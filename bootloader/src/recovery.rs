use crate::flash::flash_write;
use crate::types::{BootError, APP_BASE};
use crate::flash::flash_erase_app;
use crate::uart::{uart_getc, uart_write};

/// Reçoit 4 octets big-endian et retourne un u32
fn recv_u32() -> u32 {
    let b0 = uart_getc() as u32;
    let b1 = uart_getc() as u32;
    let b2 = uart_getc() as u32;
    let b3 = uart_getc() as u32;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

/// Mode recovery : attend une image sur UART et l'écrit en flash
pub fn recovery_mode() -> ! {
    uart_write(b"\r\n[RECOVERY] waiting for image...\r\n");

    loop {
        // 1. Recevoir la taille de l'image (4 octets big-endian)
        let size = recv_u32();

        // 2. Vérifier que la taille est raisonnable 
        if size == 0 || size > 480 * 1024 {
            uart_write(b"E: invalid size\r\n");
            continue;
        }

        // 3. Effacer le secteur app une seule fois, puis recevoir et écrire
        if let Err(_) = flash_erase_app() {
            uart_write(b"E: erase error\r\n");
            continue;
        }
        let result = recv_and_flash(APP_BASE, size);

        // 4. Répondre au PC
        match result {
            Ok(()) => uart_write(b"K\r\n"),
            Err(_) => uart_write(b"E: flash error\r\n"),
        }
    }
}

fn recv_and_flash(base: u32, size: u32) -> Result<(), BootError> {
    // Reçois les octes un et à un et écris-les en flash
    // Attention : flash_write prend un slice - utilise un buffer de 256 octets
    // et écris page par page

    // 1. Buffer de 256 octets
    let mut buffer = [0u8; 256];
    let mut offset = 0;

    // 2. Tant qu'on n'a pas reçu tous les octets
    while offset < size {
        // 3. Recevoir un octet et le stocker dans le buffer
        let byte = uart_getc();
        buffer[(offset % 256) as usize] = byte;
        offset += 1;

        // 4. Si le buffer est plein ou si c'est le dernier octet, écrire en flash
        if offset % 256 == 0 || offset == size {
            let chunk_size = if offset % 256 == 0 { 256 } else { (offset % 256) as usize };
            flash_write(base + offset - chunk_size as u32, &buffer[..chunk_size])?;
        }
    }
    Ok(())
}
