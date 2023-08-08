#![no_main]
#![no_std]

mod game;
mod display;
mod beep;

use game::*;
use display::*;
use beep::*;

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use core::cell::RefCell;
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, gpio::{Pin, Output, Level, PushPull, Disconnected}, Saadc, saadc::SaadcConfig, Timer, timer::OneShot},
    pac::{self, interrupt, TIMER0, TIMER2},
};
use embedded_hal::timer::Cancel;

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

    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut knob_pin = board.pins.p0_02.into_floating_input();

    init_display(display_timer, display_pins);
    init_beep(beep_timer, speaker_pin.degrade());

    beep();
    delay.delay_ms(250u16);
    beep();

    let tick = 50;
    let mut game = GameState::new(tick);
    loop {
        let mut raster = Raster::default();
        let k: i16 = saadc.read(&mut knob_pin).unwrap();
        let k = (k as f32 / (1 << 14) as f32).clamp(0.3, 0.7);
        let k = if k < 0.1 {
            None
        } else {
            Some(((k - 0.3) * (1.0 / 0.4)).clamp(0.0, 1.0))
        };
        if game.step(&mut raster, k) {
            break;
        }
        display_frame(&raster);
        //rprintln!("tick");
        delay.delay_ms(tick);
    }
    loop {
        asm::wfi();
    }
}
