use crate::*;

use microbit::display::nonblocking::GreyscaleImage;

/// LED array proxy for rendering.
pub type Raster = [[u8; 5]; 5];

#[macro_export]
macro_rules! microbit_display {
    ($timer:ident) => {
        pub static DISPLAY: Mutex<RefCell<Option<microbit::display::nonblocking::Display<$timer>>>> =
            Mutex::new(RefCell::new(None));

        #[interrupt]
        fn $timer() {
            cortex_m::interrupt::free(|cs| {
                if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                    d.handle_display_event();
                }
            });
        }

        pub fn init_display(timer: $timer, display_pins: microbit::gpio::DisplayPins) {
            use microbit::display::nonblocking::{Display, GreyscaleImage};

            let mut display = Display::new(timer, display_pins);
            let image = GreyscaleImage::blank();
            cortex_m::interrupt::free(|cs| {
                display.show(&image);
                *DISPLAY.borrow(cs).borrow_mut() = Some(display);
                unsafe {
                    pac::NVIC::unmask(pac::Interrupt::$timer);
                }
            });
        }
    };
}


pub fn display_frame(raster: &Raster) {
    let frame = GreyscaleImage::new(raster);
    cortex_m::interrupt::free(|cs| {
        if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            d.show(&frame);
        }
    });
}
