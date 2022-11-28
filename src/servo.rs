use arduino_hal::{
    port::{mode::Output, Pin, PinOps},
};

pub struct Servo<'a, PIN: PinOps> {
    pin: &'a mut Pin<Output, PIN>,
}

impl<'a, PIN: PinOps> Servo<'a, PIN> {
    fn servo_pwm(self, x: i32) {
        let val = (((x * 1025) / 100) + 500) as u32;

        self.pin.set_high();
        arduino_hal::delay_us(val);
        self.pin.set_low();

        arduino_hal::delay_ms(10);
    }

    fn write(self, degrees: i32) {
        let from = 90;

        if from > degrees {
            for i in (degrees..from).rev() {
                self.servo_pwm(i)
            }
        } else {
            for i in from..degrees {
                self.servo_pwm(i)
            }
        }
    }

    pub fn from_pin(pin: &'a mut Pin<Output, PIN>) -> Servo<'a, PIN> {
        Servo { pin }
    }
}
