#![no_std]
#![no_main]

mod servo;

// gyro sensor, Adafruit LSM6DSOX 6 DoF

use arduino_hal::{
    hal::port::{PB2},
    port::{mode::Output, Pin},
};
use panic_halt as _;
use servo::Servo;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn servo_pwm(x: i32, pin: &mut Pin<Output, PB2>) {
    let val = (((x * 1025) / 100) + 500) as u32;

    pin.set_high();
    arduino_hal::delay_us(val);
    pin.set_low();

    arduino_hal::delay_ms(10);
}

// Example:
// from: 180
// to: 90
fn move_servo(pin: &mut Pin<Output, PB2>, from: i32, to: i32) {
    if from > to {
        for i in (to..from).rev() {
            servo_pwm(i, pin)
        }
    } else {
        for i in from..to {
            servo_pwm(i, pin)
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    ufmt::uwriteln!(&mut serial, "Booting up...").unwrap();
    ufmt::uwriteln!(&mut serial, "Charlie R90 Flight Computer Alpha-{}", VERSION).unwrap();

    // ufmt::uwriteln!(&mut serial, "Initiating launch sequence").unwrap();
    // ufmt::uwriteln!(&mut serial, "3").unwrap();
    // arduino_hal::delay_ms(1000);
    // ufmt::uwriteln!(&mut serial, "2").unwrap();
    // arduino_hal::delay_ms(1000);
    // ufmt::uwriteln!(&mut serial, "1").unwrap();
    // arduino_hal::delay_ms(1000);
    // ufmt::uwriteln!(&mut serial, "Ignition").unwrap();
    // arduino_hal::delay_ms(500);

    let mut pwm_pin = pins.d10.into_output();

    let mut servo = Servo::from_pin(&mut pwm_pin, 90);


    loop {
        // servo_pwm(90, &mut pwm_pin);
        // ufmt::uwriteln!(&mut serial, "val: {}", 0).unwrap();
        // for i in 0..90 {
        //     servo_pwm(i, &mut pwm_pin);

        //     let val = (((i * 1025) / 100) + 500) as u32;
        //     ufmt::uwriteln!(&mut serial, "val: {}", val).unwrap();
        // }

        // servo.write(20);

        // move_servo(&mut pwm_pin, 180, 90);

        arduino_hal::delay_ms(20);

        // servo.write(180);
        // move_servo(&mut pwm_pin, 90, 180);

        arduino_hal::delay_ms(1000);

        // move_servo_to(&mut pwm_pin, 160..180, true);

        // for i in (0..90).rev() {
        //     servo_pwm(i, &mut pwm_pin);

        //     let val = (((i * 1025) / 100) + 500) as u32;
        //     ufmt::uwriteln!(&mut serial, "vala: {}", val).unwrap();
        // }

        arduino_hal::delay_ms(1000);
    }
}
