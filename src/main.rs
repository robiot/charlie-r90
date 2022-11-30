#![no_std]
#![no_main]

use arduino_hal::simple_pwm::{IntoPwmPin, Timer1Pwm};

mod servo;

// gyro sensor, Adafruit LSM6DSOX 6 DoF

use arduino_hal::{
    hal::port::PB2,
    port::{mode::Output, Pin}, simple_pwm::{Timer2Pwm, Prescaler},
};
use panic_halt as _;
use servo::{Servo, ServoPins};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Booting up...").unwrap();
    ufmt::uwriteln!(&mut serial, "Charlie R90 Flight Computer Alpha-{}", VERSION).unwrap();

    let mut servo1 = Servo::attach(ServoPins::D9, 90);

    // let mut timer = Timer1Pwm::new(dp.TC1, Prescaler::Prescale256);
    // let mut pin = pins.d9.into_output().into_pwm(&mut timer);

    // pin.enable();

    // loop {
    //     // pin.set_duty(0); // 0°
    //     // arduino_hal::delay_ms(2000u16);
    //     // arduino_hal::delay_ms(1000);

    //     // pin.set_duty(0xFF);

    //     // arduino_hal::delay_ms(1000);
    // }

    loop {
        // servo1.write(90);
        // servo2.write(170);

        arduino_hal::delay_ms(1000);

        // servo1.write(10);
        // servo2.write(90);

        arduino_hal::delay_ms(1000);
    }
}
