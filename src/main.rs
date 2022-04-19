#![no_std]
#![no_main]

use panic_halt as _; 
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    loop {
        hprintln!("Hellow World!");
    }
}
