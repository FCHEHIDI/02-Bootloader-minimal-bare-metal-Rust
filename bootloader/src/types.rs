/// En-tête de l'image applicative, placé au tout début de la zone app (0x08008000).
///
/// Le bootloader lit cette structure pour décider si l'image est valide avant de sauter.
/// `#[repr(C)]` est obligatoire : sans ça, Rust peut réordonner les champs librement,
/// ce qui corromprait la lecture depuis une adresse flash fixe.
#[repr(C)]
pub struct AppHeader {
    pub magic: u32,             // doit valoir 0xDEADBEEF
    pub version: u32,           // semver encodé (ex: 0x00010002 = v1.0.2)
    pub size: u32,              // taille de l'image en octects (hors header)
    pub crc32: u32,             // CRC32 de l'image (hors header)
    pub entry_point: u32,       // adresse de l'entrée de l'application
}

/// Valeur sentinelle attendue dans AppHeader::magic.
/// Permet de détecter immédiatement une zone flash vierge (0xFFFFFFFF) ou corrompue.
pub const APP_MAGIC: u32 = 0xDEAD_BEEF;

/// Adresse de début de la zone applicative en flash.
/// Le bootloader saute ici après validation. Défini par memory.x de app-test.
pub const APP_BASE: u32 = 0x0800_8000;

/// Erreurs possibles durant la séquence de boot.
/// `#[derive(Debug)]` permet de les afficher via defmt lors du débogage.
#[derive(Debug)]
pub enum BootError {
    InvalidMagic,
    InvalidCrc,
    ImageTooLarge,
    FlashError,
    UartError,
}