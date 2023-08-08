use crate::*;

pub const BEEP_PERIOD: u16 = 2000;

#[macro_export]
macro_rules! microbit_beep {
    ($timer:ident) => {
        pub static BEEP: cortex_m::interrupt::Mutex<RefCell<Option<Beep>>> =
            cortex_m::interrupt::Mutex::new(RefCell::new(None));

        pub struct Beep {
            beep_timer: microbit::hal::Timer<$timer, microbit::hal::timer::OneShot>,
            speaker_pin: microbit::hal::gpio::Pin<
                microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>,
            >,
            pin_high: bool,
            note_time: u32,
        }

        impl Beep {
            pub fn new(
                beep_timer: $timer,
                speaker_pin: microbit::hal::gpio::Pin<microbit::hal::gpio::Disconnected>,
            ) -> Self {
                use microbit::hal::{gpio::Level, Timer};

                Self {
                    beep_timer: Timer::new(beep_timer),
                    speaker_pin: speaker_pin.into_push_pull_output(Level::Low),
                    pin_high: false,
                    note_time: 0,
                }
            }
        }

        #[interrupt]
        fn $timer() {
            use embedded_hal::timer::Cancel;
            use microbit::hal::prelude::*;
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

        pub fn init_beep(
            beep_timer: $timer,
            speaker_pin: microbit::hal::gpio::Pin<microbit::hal::gpio::Disconnected>,
        ) {
            cortex_m::interrupt::free(|cs| {
                let mut beep = Beep::new(beep_timer, speaker_pin);
                beep.beep_timer.enable_interrupt();
                *BEEP.borrow(cs).borrow_mut() = Some(beep);

                unsafe {
                    microbit::pac::NVIC::unmask(microbit::pac::Interrupt::$timer);
                }
            });
        }
    };
}

pub fn beep() {
    use embedded_hal::prelude::*;
    cortex_m::interrupt::free(|cs| {
        if let Some(b) = BEEP.borrow(cs).borrow_mut().as_mut() {
            b.note_time = 40;
            b.beep_timer.start(BEEP_PERIOD / 2);
        }
    });
}
