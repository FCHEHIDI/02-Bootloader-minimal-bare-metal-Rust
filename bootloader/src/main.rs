#![no_std]
#![no_main]
#![allow(dead_code)]

use cortex_m_rt::entry;
#[cfg(not(test))]
use panic_halt as _;

use bootloader::boot::jump_to_app;
use bootloader::check::check_image;
use bootloader::types::APP_BASE;
use bootloader::uart::{uart_init, uart_write};
use bootloader::recovery::recovery_mode;

#[entry]
fn main() -> ! {
    // 1. Initialiser l'UART pour les logs et le recovery
    uart_init();
    uart_write(b"\r\n[BOOTLOADER] Starting...\r\n");

    // 2. Vérifier l'image applicative
    match check_image() {
        Ok(_) => {
            uart_write(b"[BOOTLOADER] Image valid, jumping to app...\r\n");
            unsafe { jump_to_app(APP_BASE) }
        }
        Err(_) => {
            uart_write(b"[BOOTLOADER] No valid image found, entering recovery mode.\r\n");
            recovery_mode();
        }
    }
}