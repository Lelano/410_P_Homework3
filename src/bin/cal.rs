#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, Saadc, saadc::SaadcConfig, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut pin = board.pins.p0_02.into_floating_input();
    let mut delay = Timer::new(board.TIMER0);

    loop {
        let k: i16 = saadc.read(&mut pin).unwrap();
        rprintln!("{}", k as f32 / (1 << 14) as f32);
        delay.delay_ms(1000u16);
    }
}
