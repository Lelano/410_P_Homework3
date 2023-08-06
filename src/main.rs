#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use core::cell::RefCell;
use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
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
    cortex_m::interrupt::free(|cs| {
        let mut display = Display::new(board.TIMER0, board.display_pins);
        let raster = [
            [0, 5, 0, 5, 0],
            [5, 9, 7, 9, 5],
            [5, 9, 9, 9, 5],
            [0, 5, 9, 5, 0],
            [0, 0, 7, 0, 0],
        ];
        let image = GreyscaleImage::new(&raster);
        display.show(&image);


        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIMER0);
        }
        pac::NVIC::unpend(pac::Interrupt::TIMER0);
    });
    loop {
        asm::nop();
    }
}
