#[repr(C)]
pub struct AppHeader {
    pub magic: u32,             // doit valoir 0xDEADBEEF
    pub version: u32,           // semver encodé (ex: 0x00010002 = v1.0.2)
    pub size: u32,              // taille de l'image en octects (hors header)
    pub crc32: u32,             // CRC32 de l'image (hors header)
    pub entry_point: u32,       // adresse de l'entrée de l'application
}

pub const APP_MAGIC: u32 = 0xDEAD_BEEF;
pub const APP_BASE: u32 = 0x0800_8000;

#[derive(Debug)]
pub enum BootError {
    InvalidMagic,
    InvalidCrc,
    ImageTooLarge,
    FlashError,
    UartError,
}