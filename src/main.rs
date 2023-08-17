//! Breakout game demo for the MicroBit v2.

#![no_main]
#![no_std]

mod beep;
mod display;
mod game;
mod knob;

use beep::{beep, BEEP_PERIOD};
use display::{display_frame, Raster};
use game::GameState;
use knob::Knob;

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use core::cell::RefCell;
use cortex_m::asm;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, Timer},
    pac::{interrupt, TIMER0, TIMER2},
};

microbit_display!(TIMER0);
microbit_beep!(TIMER2);

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Get neeeded peripherals from board.
    let board = Board::take().unwrap();
    let display_timer = board.TIMER0;
    let mut delay = Timer::new(board.TIMER1);
    let beep_timer = board.TIMER2;
    let speaker_pin = board.speaker_pin;
    let display_pins = board.display_pins;
    let saadc = board.SAADC;
    let knob_pin = board.pins.p0_02;

    // Set up our custom peripherals.
    init_display(display_timer, display_pins);
    init_beep(beep_timer, speaker_pin.degrade());
    let knob = Knob::new(saadc, knob_pin);

    // Tick time in milliseconds.
    let tick = 50;

    // Set up and run a game.
    let mut game = GameState::new(tick);
    loop {
        let mut raster = Raster::default();
        let k = knob.read();
        if game.step(&mut raster, k) {
            break;
        }
        display_frame(&raster);
        delay.delay_ms(tick);
    }

    //Game over, beep three times.
    beep();
    delay.delay_ms(250u16);
    beep();
    delay.delay_ms(250u16);
    beep();

    // Wait for a reset.
    loop {
        asm::wfi();
    }
}
