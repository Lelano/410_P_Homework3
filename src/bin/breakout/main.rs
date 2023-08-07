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
    hal::{prelude::*, gpio::{Pin, Output, Level, PushPull}, Saadc, saadc::SaadcConfig, Timer, timer::OneShot},
    pac::{self, interrupt, TIMER0, TIMER2},
};
use embedded_hal::timer::Cancel;

const BEEP_PERIOD: u16 = 2000;

static BEEP: Mutex<RefCell<Option<Beep>>> = Mutex::new(RefCell::new(None));

static DISPLAY: Mutex<RefCell<Option<Display<TIMER0>>>> = Mutex::new(RefCell::new(None));

struct Beep {
    beep_timer: Timer<TIMER2, OneShot>,
    speaker_pin: Pin<Output<PushPull>>,
    pin_high: bool,
    note_time: u32,
}

impl Beep {
    pub fn new(beep_timer: Timer<TIMER2, OneShot>, speaker_pin: Pin<Output<PushPull>>) -> Self {
        Self {
            beep_timer,
            speaker_pin,
            pin_high: false,
            note_time: 0,
        }
    }
}

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            d.handle_display_event();
        }
    });
}

#[interrupt]
fn TIMER2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(b) = BEEP.borrow(cs).borrow_mut().as_mut() {
            if b.note_time == 0 {
                b.beep_timer.cancel().unwrap();
                return;
            }
            if b.pin_high {
                b.speaker_pin.set_low().unwrap();
            } else {
                b.speaker_pin.set_high().unwrap();
            }
            b.pin_high = !b.pin_high;
            b.note_time -= 1;
            b.beep_timer.cancel().unwrap();
            b.beep_timer.start(BEEP_PERIOD / 2);
        }
    });
}

pub fn beep() {
    cortex_m::interrupt::free(|cs| {
        if let Some(b) = BEEP.borrow(cs).borrow_mut().as_mut() {
            b.note_time = 40;
            b.beep_timer.start(BEEP_PERIOD / 2);
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut delay = Timer::new(board.TIMER1);
    let beep_timer = Timer::new(board.TIMER2);
    let speaker_pin = board.speaker_pin.into_push_pull_output(Level::Low);
    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut knob_pin = board.pins.p0_02.into_floating_input();

    cortex_m::interrupt::free(|cs| {
        let mut display = Display::new(board.TIMER0, board.display_pins);
        let image = GreyscaleImage::blank();
        display.show(&image);
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);

        let mut beep = Beep::new(beep_timer, speaker_pin.degrade());
        beep.beep_timer.enable_interrupt();
        *BEEP.borrow(cs).borrow_mut() = Some(beep);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIMER0);
            pac::NVIC::unmask(pac::Interrupt::TIMER2);
        }
        pac::NVIC::unpend(pac::Interrupt::TIMER0);
        pac::NVIC::unpend(pac::Interrupt::TIMER2);
    });

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
