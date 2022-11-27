#![no_std]
#![no_main]

use arduino_hal::{

    Peripherals, Pins
};
use panic_halt as _;

struct ChairlieFlightComputer {
    dp: Peripherals,
    pins: Pins,
}

impl ChairlieFlightComputer {
    fn setup(self) {
        let mut serial = arduino_hal::default_serial!(self.dp, self.pins, 57600);
    
        ufmt::uwriteln!(&mut serial, "Booting up...").unwrap();
        ufmt::uwriteln!(&mut serial, "Charlie R90 Flight Computer Alpha-1.0.0").unwrap();    
    }

    // fn service_loop(self) {

    // }

    fn init(self) {
        let mut serial = arduino_hal::default_serial!(self.dp, self.pins, 57600);
    
        ufmt::uwriteln!(&mut serial, "Booting up...").unwrap();

        // self.setup();

        // loop {
        //     self.service_loop();
        // }
    }

    /**
     * Create new struct
     */
    pub fn new() -> Self {
        let pins = arduino_hal::pins!(arduino_hal::Peripherals::take().unwrap());
        ChairlieFlightComputer { dp: arduino_hal::Peripherals::take().unwrap(), pins }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    ChairlieFlightComputer::new().init();
    loop {}
    // let dp = arduino_hal::Peripherals::take().unwrap();
    // let pins = arduino_hal::pins!(dp);

    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // let mut pwm_pin = pins.d9.into_output();

    // ufmt::uwriteln!(&mut serial, "Booting up...").unwrap();
    // ufmt::uwriteln!(&mut serial, "Charlie R90 Flight Computer Alpha-1.0.0").unwrap();

    // ufmt::uwriteln!(&mut serial, "Initiating launch sequence").unwrap();
    // ufmt::uwriteln!(&mut serial, "3").unwrap();
    // arduino_hal::delay_ms(1000);
    // ufmt::uwriteln!(&mut serial, "2").unwrap();
    // arduino_hal::delay_ms(1000);
    // ufmt::uwriteln!(&mut serial, "1").unwrap();
    // arduino_hal::delay_ms(1000);
    // ufmt::uwriteln!(&mut serial, "Ignition").unwrap();
    // arduino_hal::delay_ms(500);

    // loop {
        // led.set_high();
        // arduino_hal::delay_us(500);
        // led.set_low();
        // arduino_hal::delay_ms(20);
    // }

    // Important because this sets the bit in the DDR register!

    // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
    // - Each count increases the duty-cycle by 4us.
    // - Use OC1A which is connected to D9 of the Arduino Uno.
    // let tc1 = dp.TC1;
    // tc1.icr1.write(|öw| unsafe { w.bits(4999) });

    // tc1.tccr1a
    //     .write(|w| w.wgm1().bits(2).com1a().match_clear());
    // tc1.tccr1b
    //     .write(|w| w.wgm1().bits(3).cs1().prescale_64());

    // loop {
    // 100 counts => 0.4ms
    // 700 counts => 2.8ms
    // for duty in 100..=700 {
    //     tc1.ocr1a.write(|w| unsafe { w.bits(duty) });
    //     arduino_hal::delay_ms(20);
    // }
    // }
}
