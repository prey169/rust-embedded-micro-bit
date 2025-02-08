#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayUs;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

// const LEDS = [(0,0), (0,1), (0,2) (0,3) (0,4), (1,4), (2,4), (3,4), (4,4), (4,3)]

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut past_led = (0, 0);

    let rows = leds.len() - 1;
    let cols = leds[0].len() - 1;
    let time = 100;

    loop {
        let mut row = 0;
        let mut col = 0;
        let mut i = 0;
        while col < cols {
            leds[row][col] = 1;
            leds[past_led.0][past_led.1] = 0;
            past_led = (row, col);
            display.show(&mut timer, leds, time);
            col += 1;
        }
        while row < rows {
            leds[row][col] = 1;
            leds[past_led.0][past_led.1] = 0;
            past_led = (row, col);
            display.show(&mut timer, leds, time);
            row += 1;
        }

        while col > 0 {
            leds[row][col] = 1;
            leds[past_led.0][past_led.1] = 0;
            past_led = (row, col);
            display.show(&mut timer, leds, time);
            col -= 1;
        }
        while row > 0 {
            leds[row][col] = 1;
            leds[past_led.0][past_led.1] = 0;
            past_led = (row, col);
            display.show(&mut timer, leds, time);
            row -= 1;
        }
    }
}
