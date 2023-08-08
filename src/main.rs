#![no_main]
#![no_std]

mod beep;
mod display;
mod game;
mod knob;

use beep::*;
use display::*;
use game::*;
use knob::*;

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
    let board = Board::take().unwrap();
    let display_timer = board.TIMER0;
    let mut delay = Timer::new(board.TIMER1);
    let beep_timer = board.TIMER2;
    let speaker_pin = board.speaker_pin;
    let display_pins = board.display_pins;
    let saadc = board.SAADC;
    let knob_pin = board.pins.p0_02;

    init_display(display_timer, display_pins);
    init_beep(beep_timer, speaker_pin.degrade());
    let knob = Knob::new(saadc, knob_pin);

    beep();
    delay.delay_ms(250u16);
    beep();

    let tick = 50;
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
    loop {
        asm::wfi();
    }
}
