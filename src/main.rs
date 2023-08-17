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
    // Buttong tracking, blp = button last postition
    // bnp = button next position.
    let mut blp:f32 = 0.5f32;
    let mut bnp:f32 = 0.5f32;
    // Knob tracking
    let mut knob_last:Option<f32> = None;
    let mut knob_active:bool = true;

    // Tick time in milliseconds.
    let tick = 50;
    // Set up and run a game.
    let mut game = GameState::new(tick);

    loop {

        let mut raster = Raster::default();
        let k = knob.read();

        // Checks that k and knob_last are not none, then if the 
        // k is 0.05 or more away from knob_last, knob control is
        // considered active. A range is neccesary becase the voltage 
        // fluctuates on the pin.
        if let Some(curr_k) = k {
             if let Some(curr_kl) = knob_last {
                 if (curr_kl + 0.05 < curr_k ) || (curr_kl - 0.05 > curr_k ) {
                     knob_active = true;
                 } else {
                     knob_active = false;
                 }
             }
        }
        
        knob_last = k;
        
        // Check if buttons are being pressed.
        // If the knob is off and a button is pressed
        // the control switches to buttons.
        if board.buttons.button_a.is_low().unwrap() {
            if k == None {knob_active = false;}
            if blp > 0.1 {bnp = blp - 0.1;} 
        }
        if board.buttons.button_b.is_low().unwrap() {
            if k == None {knob_active = false;}
            if blp < 0.9 {bnp = blp + 0.1;}
        }

        blp = bnp;

        // Set the last knob position.
        if knob_active {
            if let Some(curr_k) = k {
                blp = curr_k;
                bnp = curr_k;
            }
            
        } 

        if knob_active {
            if game.step(&mut raster, k) {
                break;
            }
        } else {
            if game.step(&mut raster, Some(bnp)) {
                break;
            }
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
