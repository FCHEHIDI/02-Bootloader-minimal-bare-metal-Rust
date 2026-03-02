#![no_std]
#![no_main]
#![allow(dead_code)]

use cortex_m_rt::entry;
#[cfg(not(test))]
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {}
}