#![no_std]
#![no_main]

mod types;

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {}
}