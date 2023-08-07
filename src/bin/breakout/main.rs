#![no_main]
#![no_std]

mod game;
use game::*;

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use core::cell::RefCell;
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{prelude::*, Saadc, saadc::SaadcConfig, Timer},
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
    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut knob_pin = board.pins.p0_02.into_floating_input();

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
        let frame = GreyscaleImage::new(&raster);
        cortex_m::interrupt::free(|cs| {
            let mut d = DISPLAY.borrow(cs).borrow_mut();
            d.as_mut().unwrap().show(&frame);
        });
        //rprintln!("tick");
        delay.delay_ms(tick);
    }
    loop {
        asm::wfi();
    }
}
