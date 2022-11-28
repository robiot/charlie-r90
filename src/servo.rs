use arduino_hal::{
    port::{mode::Output, Pin, PinOps},
};

// https://github.com/arduino-libraries/Servo/blob/master/src/avr/Servo.cpp
// https://github.com/Rahix/avr-hal/issues/127

pub struct Servo<'a, PIN: PinOps> {
    pin: &'a mut Pin<Output, PIN>,
    last_to: i32,
}

impl<'a, PIN: PinOps> Servo<'a, PIN> {
    fn servo_pwm(&mut self, x: i32) {
        let val = (((x * 1025) / 100) + 544) as u32;

        self.pin.set_high();
        arduino_hal::delay_us(val);
        self.pin.set_low();

        arduino_hal::delay_ms(10);
    }

    pub fn write(&mut self, degrees: i32) {
        let from = self.last_to;

        if from > degrees {
            for i in (degrees..from).rev() {
                self.servo_pwm(i);
            }
        } else {
            for i in from..degrees {
                self.servo_pwm(i)
            }
        }
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
    pub fn from_pin(pin: &'a mut Pin<Output, PIN>, initial_degrees: i32) -> Servo<'a, PIN> {
        let mut cself = Servo { pin: pin, last_to: initial_degrees };
        
        for i in 0..90 {
            cself.servo_pwm(i);
        }
        cself
    }
}
