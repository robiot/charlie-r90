use arduino_hal::{
    hal::port::{PB2, PB1},
    port::{mode::Output, Pin},
};
use embedded_hal::PwmPin;

pub trait ServoPinTrait {}

impl ServoPinTrait for PB2 {}
impl ServoPinTrait for PB1 {}

pub struct Servo<T: PwmPin> {
    pin: Pin<Output, T>,
}

impl<T: ServoPinTrait> Servo<T> {
    fn servo_pwm(self, x: i32, pin: &mut Pin<Output, T>) {
        let val = (((x * 1025) / 100) + 500) as u32;

        pin.set_high();
        arduino_hal::delay_us(val);
        pin.set_low();

        arduino_hal::delay_ms(10);
    }

    fn write(self, degrees: i32) {
        let from = 90;

        if from > degrees {
            for i in (degrees..from).rev() {
                self.servo_pwm(i, &mut self.pin)
            }
        } else {
            for i in from..degrees {
                self.servo_pwm(i, &mut self.pin)
            }
        }
    }

    pub fn from_pin(pin: Pin<Output, T>) -> Servo<T> {
        Servo { pin }
    }
}
