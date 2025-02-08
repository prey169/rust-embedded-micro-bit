#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit as _;
use panic_rtt_target as _;
//use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;
    //rtt_init_print!();
    //rprintln!("Hello World");
    //
    loop {}
}
