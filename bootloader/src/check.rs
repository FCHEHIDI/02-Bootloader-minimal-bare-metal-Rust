use crate::crc::crc32;
use crate::types::{AppHeader, BootError, APP_BASE, APP_MAGIC};
use core::mem::size_of;

pub fn check_image() -> Result<&'static AppHeader, BootError> {
    // 1. Lire le header depuis APP_BASE
    let header = unsafe { &*(APP_BASE as *const AppHeader)};

    // 2. Vérifier le magic
    if header.magic != APP_MAGIC {
        return Err(BootError::InvalidMagic);
    }

    // 3. Vérifier que la taille est raisonnable ( < 480K)
    if header.size as usize > 480 * 1024 {
        return Err(BootError::ImageTooLarge);
    }

    // 4. Calculer le CRC32 sur les octets après le header
    let data = unsafe {
        core::slice::from_raw_parts(
            (APP_BASE as usize + size_of::<AppHeader>()) as *const u8,
            header.size as usize,
        )
    };
    let crc = crc32(data);

    // 5. Comparer avec header.crc32
    if crc != header.crc32 {
        return Err(BootError::InvalidCrc);
    }

    // Retourner Ok(&header) ou Err(...)
    Ok(&header)
}