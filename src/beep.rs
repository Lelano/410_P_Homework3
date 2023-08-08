use crate::*;

pub const BEEP_PERIOD: u16 = 2000;

#[macro_export]
macro_rules! microbit_beep {
    ($timer:ident) => {
        pub static BEEP: Mutex<RefCell<Option<Beep>>> = Mutex::new(RefCell::new(None));

        pub struct Beep {
            beep_timer: Timer<$timer, OneShot>,
            speaker_pin: Pin<Output<PushPull>>,
            pin_high: bool,
            note_time: u32,
        }

        impl Beep {
            pub fn new(beep_timer: $timer, speaker_pin: Pin<Disconnected>) -> Self {
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

        pub fn init_beep(beep_timer: $timer, speaker_pin: Pin<Disconnected>) {
            cortex_m::interrupt::free(|cs| {
                let mut beep = Beep::new(beep_timer, speaker_pin);
                beep.beep_timer.enable_interrupt();
                *BEEP.borrow(cs).borrow_mut() = Some(beep);

                unsafe {
                    pac::NVIC::unmask(pac::Interrupt::$timer);
                }
            });
        }
    };
}

pub fn beep() {
    cortex_m::interrupt::free(|cs| {
        if let Some(b) = BEEP.borrow(cs).borrow_mut().as_mut() {
            b.note_time = 40;
            b.beep_timer.start(BEEP_PERIOD / 2);
        }
    });
}
