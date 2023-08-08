//! Generic MicroBit v2 nonblocking display handler.

use crate::*;

use microbit::display::nonblocking::GreyscaleImage;

/// LED array proxy for rendering.
pub type Raster = [[u8; 5]; 5];

/// Macro for declaring the stuff needed for the non-blocking display.
///
/// Argument is which timer to be used, in all caps: for example, `TIMER0`.
#[macro_export]
macro_rules! microbit_display {
    ($timer:ident) => {
        /// Global state of display.
        pub static DISPLAY: cortex_m::interrupt::Mutex<
            RefCell<Option<microbit::display::nonblocking::Display<$timer>>>,
        > = cortex_m::interrupt::Mutex::new(RefCell::new(None));

        /// Display timer handler.
        #[interrupt]
        fn $timer() {
            cortex_m::interrupt::free(|cs| {
                if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                    d.handle_display_event();
                }
            });
        }

        /// Set up the display. This must be called before first use.
        pub fn init_display(timer: $timer, display_pins: microbit::gpio::DisplayPins) {
            use microbit::display::nonblocking::{Display, GreyscaleImage};

            let mut display = Display::new(timer, display_pins);
            let image = GreyscaleImage::blank();
            cortex_m::interrupt::free(|cs| {
                display.show(&image);
                *DISPLAY.borrow(cs).borrow_mut() = Some(display);
                unsafe {
                    microbit::pac::NVIC::unmask(microbit::pac::Interrupt::$timer);
                }
            });
        }
    };
}

/// Display a frame.
pub fn display_frame(raster: &Raster) {
    let frame = GreyscaleImage::new(raster);
    cortex_m::interrupt::free(|cs| {
        if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            d.show(&frame);
        }
    });
}
