use crate::*;

use microbit::hal::{
    gpio::{p0::P0_02, Disconnected, Floating, Input},
    pac::SAADC,
    prelude::*,
    saadc::SaadcConfig,
    Saadc,
};

pub struct Knob {
    saadc: RefCell<Saadc>,
    knob_pin: RefCell<P0_02<Input<Floating>>>,
}

impl Knob {
    pub fn new(saadc: SAADC, knob_pin: P0_02<Disconnected>) -> Self {
        let saadc = Saadc::new(saadc, SaadcConfig::default());
        Self {
            saadc: RefCell::new(saadc),
            knob_pin: RefCell::new(knob_pin.into_floating_input()),
        }
    }

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
