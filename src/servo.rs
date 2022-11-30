use arduino_hal::pac::TC1;

// https://github.com/arduino-libraries/Servo/blob/master/src/avr/Servo.cpp
// https://github.com/Rahix/avr-hal/issues/127

// TODO: make it support both 9 and 10 pin, possibly 6, 5, 11, 3: https://docs.arduino.cc/tutorials/generic/secrets-of-arduino-pwm

pub enum ServoPins {
    // D6,  // OC0A
    // D5,  // 0C0B
    D9, // 0C1A
    D10, // 0C1B
        // D11, // 0C2A
        // D3,  // 0C2B
}

pub struct Servo {
    pin: ServoPins,
    last_to: i32,
    tc1: TC1,
}

impl Servo {
    pub fn write(&mut self, degrees: i32) {
        let mut degrees = if degrees > 180 {
            180
        } else if degrees < 0 {
            0
        } else {
            degrees
        };

        let servo_up_time_ms = (degrees as f64) * 0.011111 + 0.5;

        // - Each count increases the duty-cycle by 4us = 0.004ms.
        let value_to_write = (servo_up_time_ms / 0.004) as u16;

        // Writes the new time to be HIGH to the representing oscillator.
        match self.pin {
            ServoPins::D9 => 
                self.tc1.ocr1a.write(|w| unsafe { w.bits(value_to_write) }),
            ServoPins::D10 => self.tc1.ocr1b.write(|w| unsafe { w.bits(value_to_write) }),
        }

        self.last_to = degrees;
    }

    /**
     * Creates an servo instance for specified pin
     * Basically like the attach function in Servo.h
     */
    pub fn attach(pin: ServoPins, initial_degrees: i32) -> Servo {
        let dp = unsafe { arduino_hal::Peripherals::steal() };
        let pins = arduino_hal::pins!(dp);

        // Initialize the oscillator
        // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
        let tc1 = dp.TC1;
        tc1.icr1.write(|w| unsafe { w.bits(4999) });

        tc1.tccr1b
            .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

        // Set the used pin to output mode, write to tccr1a with the corresponding compare output
        match pin {
            ServoPins::D9 => {
                pins.d9.into_output();
                tc1.tccr1a
                    .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
            }
            ServoPins::D10 => {
                pins.d10.into_output();
                tc1.tccr1a
                    .write(|w| w.wgm1().bits(0b10).com1b().match_clear());
            }
        }

        let mut servo = Servo {
            pin,
            last_to: initial_degrees,
            tc1,
        };

        // Write initial degrees
        servo.write(initial_degrees);

        servo
    }
}
