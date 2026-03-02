#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
const GPIOA_MODER: *mut u32 = 0x4002_0000 as *mut u32;
const GPIOA_ODR: *mut u32 = 0x4002_0014 as *mut u32;

#[entry]
fn main() -> ! {
    unsafe {
        // Init une seule fois
        let rcc = core::ptr::read_volatile(RCC_AHB1ENR);
        core::ptr::write_volatile(RCC_AHB1ENR, rcc | 1);
        let moder = core::ptr::read_volatile(GPIOA_MODER);
        core::ptr::write_volatile(GPIOA_MODER, (moder & !(0b11 << 10)) | (0b01 << 10));
    }

    loop {
        unsafe {
            let odr = core::ptr::read_volatile(GPIOA_ODR);
            core::ptr::write_volatile(GPIOA_ODR, odr ^ (1 << 5));
        }
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}