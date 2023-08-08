//! Treat an externally connected potentiometer as a control
//! knob.  The pot center tap should be connected to Ring 0
//! on the MicroBit v2 edge connector, with the other inputs
//! attached to power and ground in such a way that
//! clockwise motion increases voltage.

use crate::*;

use microbit::hal::{
    gpio::{p0::P0_02, Disconnected, Floating, Input},
    pac::SAADC,
    prelude::*,
    saadc::SaadcConfig,
    Saadc,
};

/// Material needed for doing knob reads.
pub struct Knob {
    saadc: RefCell<Saadc>,
    knob_pin: RefCell<P0_02<Input<Floating>>>,
}

impl Knob {
    /// Set up to read the knob.
    pub fn new(saadc: SAADC, knob_pin: P0_02<Disconnected>) -> Self {
        let saadc = Saadc::new(saadc, SaadcConfig::default());
        Self {
            saadc: RefCell::new(saadc),
            knob_pin: RefCell::new(knob_pin.into_floating_input()),
        }
    }

    /// Read the knob. Returns `Some` fraction 0.0..1.0 of
    /// knob rotation if the knob is 0.3..0.7 full voltage,
    /// clamping on the left and right. However, returns
    /// `None` if the knob is less than 0.1 full voltage.
    pub fn read(&self) -> Option<f32> {
        let mut knob_pin = self.knob_pin.borrow_mut();
        let k: i16 = self.saadc.borrow_mut().read(&mut *knob_pin).unwrap();
        let k = k as f32 / (1 << 14) as f32;
        if k < 0.1 {
            None
        } else {
            let k = k.clamp(0.3, 0.7);
            Some(((k - 0.3) * (1.0 / 0.4)).clamp(0.0, 1.0))
        }
    }
}
