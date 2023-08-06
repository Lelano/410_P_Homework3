#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use libm::*;
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


    let tick = 100u8;
    let _paddle_width = 1.5;

    let mut _blocks = [[2; 5]; 2];
    let mut ball_position = (2.0, 3.0);
    let ball_direction = (1.0, 0.0);
    let ball_velocity = 0.1;
    let mut _paddle_position = 2.5;
    let mut _paddle_velocity = 0.3;
    let mut _ball_count = 0;

    loop {
        let mut raster = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        
        let round = |v: f32| floorf(v + 0.5).clamp(0.0, 4.0) as usize;

        let (r, c) = ball_position;
        let (r, c) = (round(r), round(c));
        raster[r][c] = 9;
        let image = GreyscaleImage::new(&raster);
        cortex_m::interrupt::free(|cs| {
            if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                d.show(&image);
            }
        });
                                  
        let (r, c) = ball_position;
        let (dr, dc) = ball_direction;
        let (r, c) = (
            r + dr * ball_velocity,
            c + dc * ball_velocity,
        );
        ball_position = (r, c);

        delay.delay_ms(tick);
    }
}
