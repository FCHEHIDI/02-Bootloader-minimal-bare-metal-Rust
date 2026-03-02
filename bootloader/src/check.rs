use crate::crc::crc32;
use crate::types::{AppHeader, BootError, APP_BASE, APP_MAGIC};
use core::mem::size_of;

/// Vérifie l'intégrité de l'image applicative en flash.
///
/// Séquence :
/// 1. Cast de l'adresse APP_BASE en référence AppHeader (unsafe, adresse fixe connue)
/// 2. Vérification du magic 0xDEADBEEF
/// 3. Vérification que la taille déclarée est raisonnable (< 480 Ko)
/// 4. Calcul du CRC32 sur les octets de l'image (hors header)
/// 5. Comparaison avec le CRC32 stocké dans le header
pub fn check_image() -> Result<&'static AppHeader, BootError> {
    // SAFETY: APP_BASE est l'adresse de début de la zone flash app, toujours mapped.
    // La référence 'static est valide car la flash ne bouge pas.
    let header = unsafe { &*(APP_BASE as *const AppHeader)};

    // 2. Vérifier le magic
    if header.magic != APP_MAGIC {
        return Err(BootError::InvalidMagic);
    }

    // 3. Vérifier que la taille est raisonnable ( < 480K)
    if header.size as usize > 480 * 1024 {
        return Err(BootError::ImageTooLarge);
    }

    // 4. Construire un slice sur les octets de l'image, juste après le header.
    // SAFETY: la taille a été validée à l'étape 3, l'adresse est dans la flash mappée.
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