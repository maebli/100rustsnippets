#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

#[entry]
fn main() -> ! {
    info!("Hello World!");
    loop {}
}
