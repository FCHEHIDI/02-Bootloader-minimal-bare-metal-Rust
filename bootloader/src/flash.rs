use crate::types::{BootError, APP_BASE};

// Registres du contrôleur Flash STM32F411 — RM0383 section 3
const FLASH_KEYR: *mut u32 = 0x4002_3C04 as *mut u32; // Flash key register : déverrouillage séquentiel
const FLASH_SR:   *mut u32 = 0x4002_3C0C as *mut u32; // Flash status register (BSY = bit 16)
const FLASH_CR:   *mut u32 = 0x4002_3C10 as *mut u32; // Flash control register (PG=bit0, SER=bit1, SNB=bits6:3, STRT=bit16, LOCK=bit31)

// Clés de déverrouillage — valeurs imposées par le hardware, dans cet ordre précis
const KEY1: u32 = 0x4567_0123;
const KEY2: u32 = 0xCDEF_89AB;

/// Déverouille la flash pour écriture
unsafe fn flash_unlock() {
    core::ptr::write_volatile(FLASH_KEYR, KEY1);
    core::ptr::write_volatile(FLASH_KEYR, KEY2);
}

/// Reverouille la flash
unsafe fn flash_lock() {
    // Bit 31 de FLASH_CR = LOCK
    let cr = core::ptr::read_volatile(FLASH_CR);
    core::ptr::write_volatile(FLASH_CR, cr | (1 << 31));
}

/// Attend que la flash soit prâte (bit BSY = bit 16 de SR)
unsafe fn flash_wait_ready() {
    while core::ptr::read_volatile(FLASH_SR) & (1 << 16) != 0 {}
}

/// Efface un secteur entier.
///
/// Composition de FLASH_CR :
/// - bits 6:3 (SNB) = numéro du secteur
/// - bit 1 (SER)    = sector erase mode
/// - bit 16 (STRT)  = démarrer l'effacement
///
/// Un secteur effacé a tous ses bits à 1 (0xFF).
/// On ne peut écrire qu'en passant des bits de 1 à 0 — l'inverse nécessite un nouvel effacement.
unsafe fn flash_erase_sector(sector: u8) -> Result<(), BootError> {
    flash_wait_ready();
    let cr = (sector as u32) << 3 | (1 << 1) | (1 << 16);
    core::ptr::write_volatile(FLASH_CR, cr);
    flash_wait_ready();
    Ok(())
}

/// Convertit une adresse flash en numéro de secteur STM32F411
fn addr_to_sector(addr: u32) -> Result<u8, BootError> {
    match addr {
        0x0800_8000..=0x0800_BFFF => Ok(2),
        0x0800_C000..=0x0800_FFFF => Ok(3),
        0x0801_0000..=0x0801_FFFF => Ok(4),
        0x0802_0000..=0x0803_FFFF => Ok(5),
        0x0804_0000..=0x0805_FFFF => Ok(6),
        0x0806_0000..=0x0807_FFFF => Ok(7),
        _ => Err(BootError::FlashError),
    }
}

/// Efface le secteur correspondant à l'adresse de base de l'app
pub fn flash_erase_app() -> Result<(), BootError> {
    let sector = addr_to_sector(APP_BASE)?;
    unsafe { flash_erase_sector(sector) }
}

/// Écrit un slice d'octets en flash octet par octet.
///
/// Précondition : le secteur cible doit avoir été effacé avec `flash_erase_app()`.
/// Écrire sur un secteur non effacé corrompt silencieusement les données
/// (les bits ne peuvent passer que de 1→0, jamais de 0→1 sans effacement).
///
/// Séquence par octet :
/// 1. Activer PG (Program) dans FLASH_CR
/// 2. Écrire l'octet à l'adresse cible en *mut u8 (pas u32 !)
/// 3. Attendre BSY = 0
/// 4. Désactiver PG
pub fn flash_write(addr: u32, data: &[u8]) -> Result<(), BootError> {
    unsafe {
        flash_unlock();
        for (i, &byte) in data.iter().enumerate() {
            flash_wait_ready();
            // Activer le mode Program (PSIZE=00 pour écriture octet par octet)
            let cr = core::ptr::read_volatile(FLASH_CR);
            core::ptr::write_volatile(FLASH_CR, cr | (1 << 0));
            // Écrire en *mut u8 — écriture u32 écraserait 4 octets
            core::ptr::write_volatile((addr + i as u32) as *mut u8, byte);
            flash_wait_ready();
            // Désactiver PG après chaque octet
            let cr = core::ptr::read_volatile(FLASH_CR);
            core::ptr::write_volatile(FLASH_CR, cr & !(1 << 0));
        }
        flash_lock();
    }
    Ok(())
}
