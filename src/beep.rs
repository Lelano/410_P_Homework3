//! Provide the ability to asynchronously beep the MicroBit
//! v2 speaker.  The beep frequency is currently hardwired
//! to 500Hz.

use crate::*;

/// Length of one cycle of beep in milliseconds.
pub const BEEP_PERIOD: u16 = 2000;

/// Length of beep in cycles.
pub const BEEP_TIME: u32 = 20;

/// Macro for declaring the stuff needed for beeping.
///
/// Argument is which timer to be used, in all caps: for example, `TIMER2`.
#[macro_export]
macro_rules! microbit_beep {
    ($timer:ident) => {
        /// Global state of beep.
        pub static BEEP: cortex_m::interrupt::Mutex<RefCell<Option<Beep>>> =
            cortex_m::interrupt::Mutex::new(RefCell::new(None));

        /// Beep status elements.
        pub struct Beep {
            /// Timer used for beeping.
            beep_timer: microbit::hal::Timer<$timer, microbit::hal::timer::OneShot>,
            /// Pin for audio output.
            speaker_pin: microbit::hal::gpio::Pin<
                microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>,
            >,
            /// Mirror of speaker pin state: `true` iff speaker pin is high.
            pin_high: bool,
            /// Remaining time of beep in cycles. When 0,
            /// beep is not sounding.
            note_time: u32,
        }

        impl Beep {
            /// Make a new structure for handling beeps.
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

        /// Handle beep interrupt.
        #[interrupt]
        fn $timer() {
            use embedded_hal::timer::Cancel;
            use microbit::hal::prelude::*;
            cortex_m::interrupt::free(|cs| {
                if let Some(b) = BEEP.borrow(cs).borrow_mut().as_mut() {
                    if b.note_time == 0 {
                        // No beep is running.
                        b.beep_timer.cancel().unwrap();
                        return;
                    }

                    // Cycle the beep.
                    if b.pin_high {
                        b.speaker_pin.set_low().unwrap();
                    } else {
                        b.speaker_pin.set_high().unwrap();
                        b.note_time -= 1;
                    }
                    b.pin_high = !b.pin_high;

                    // Restart the beep timer. Cancellation
                    // is necessary to clear the timer
                    // interrupt event.
                    b.beep_timer.cancel().unwrap();
                    b.beep_timer.start(BEEP_PERIOD / 2);
                }
            });
        }

        /// Set up the beep system.
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

/// Start a beep. This function is asynchronous: it returns
/// immediately.
pub fn beep() {
    use embedded_hal::prelude::*;
    cortex_m::interrupt::free(|cs| {
        if let Some(b) = BEEP.borrow(cs).borrow_mut().as_mut() {
            b.note_time = BEEP_TIME;
            b.beep_timer.start(BEEP_PERIOD / 2);
        }
    });
}
