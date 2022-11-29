
use arduino_hal::{
    port::{mode::Output, Pin, PinOps}, pac::TC1,
};

// https://github.com/arduino-libraries/Servo/blob/master/src/avr/Servo.cpp
// https://github.com/Rahix/avr-hal/issues/127

pub struct Servo<'a, PIN: PinOps> {
    pin: &'a mut Pin<Output, PIN>,
    last_to: f64,
    tc1: TC1,
}

impl<'a, PIN: PinOps> Servo<'a, PIN> {
    pub fn write(&mut self, degrees: f64) {
        let val = degrees * 0.011111 + 0.5;
        self.tc1.ocr1b.write(|w| unsafe { w.bits((val / 0.004) as u16) });

        self.last_to = degrees;
    }

    /**
     * Creates an servo instance for specified pin
     * ```
     * let mut pwm_pin = pins.d10.into_output();
     * let mut servo = Servo::from_pin(&mut pwm_pin);
     * ```
     * Basically like the attach function in Servo.h
     */
    pub fn from_pin(pin: &'a mut Pin<Output, PIN>, initial_degrees: f64) -> Servo<'a, PIN> {
        let dp = arduino_hal::Peripherals::take().unwrap();

        // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
        // - Each count increases the duty-cycle by 4us = 0.004ms.
        let tc1 = dp.TC1;
        tc1.icr1.write(|w| unsafe { w.bits(4999) });
        tc1.tccr1a
            .write(|w| w.wgm1().bits(0b10).com1b().match_clear());
        tc1.tccr1b
            .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

        // Initialize
        let mut cself = Servo { pin: pin, last_to: initial_degrees, tc1 };
        
        // Write initial degrees
        cself.write(initial_degrees);

        cself
    }
}
