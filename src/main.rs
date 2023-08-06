#![allow(clippy::assign_op_pattern)]
#![no_main]
#![no_std]

mod breakout;
use breakout::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{prelude::*, Timer},
    pac::{self, interrupt, TIMER0},
};

static DISPLAY: Mutex<RefCell<Option<Display<TIMER0>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            d.handle_display_event();
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut delay = Timer::new(board.TIMER1);

    cortex_m::interrupt::free(|cs| {
        let mut display = Display::new(board.TIMER0, board.display_pins);
        let image = GreyscaleImage::blank();
        display.show(&image);

        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIMER0);
        }
        pac::NVIC::unpend(pac::Interrupt::TIMER0);
    });


    let tick = 250;
    loop {
        rprintln!("start");
        let mut game = GameState::default();
        loop {
            let mut raster = Raster::default();
            if game.step(&mut raster, tick) {
                break;
            }
            let frame = GreyscaleImage::new(&raster);
            cortex_m::interrupt::free(|cs| {
                let mut d = DISPLAY.borrow(cs).borrow_mut();
                d.as_mut().unwrap().show(&frame);
            });
            //rprintln!("tick");
            delay.delay_ms(tick);
        }
    }
}
