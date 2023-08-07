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
    let buttons = [
        board.buttons.button_a.degrade(),
        board.buttons.button_b.degrade(),
    ];

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
        let bs = core::array::from_fn(|i| {
            buttons[i].is_low().unwrap()
        });
        if game.step(&mut raster, Some(bs)) {
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
