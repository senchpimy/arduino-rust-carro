/*!
 * Blink the builtin LED - the "Hello World" of embedded programming.
 */
#![no_std]
#![no_main]

use panic_halt as _;

const WAIT_BETWEEN_ACTIONS: u16 = 1000u16;
const MINIMAL_DISTANCE: u16 = 10u16;
const ACCEPTABLE_DISTANCE: u16 = 10u16;
const TRIGGER_UP_TIME: u16 = 10u16;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer = dp.TC1;
    timer.tccr1b.write(|w| w.cs1().prescale_64());

    let left_forw = pins.d4.into_output().downgrade();
    let left_back = pins.d5.into_output().downgrade();
    let right_forw = pins.d6.into_output().downgrade();
    let right_back = pins.d7.into_output().downgrade();
    let mut trig = pins.d13.into_output();
    let echo = pins.d12;

     loop{
         let mut  distancia=0;
        let mut delay = arduino_hal::Delay::new();
        timer.tcnt1.write(|w| w.bits(0) );
    
        trig.set_high();
        delay.delay_us(TRIGGER_UP_TIME);
        trig.set_low();
    
        while echo.is_low() {
            if timer.tcnt1.read().bits() >= 65000 {
                distancia = 63500;
            }
        }
    
        // restarting the timer by writing 0 bits to it
        timer.tcnt1.write(|w| w.bits(0) );
    
        // waiting for the echo to get low again
        while echo.is_high() {}
    
        // Taking the time the echo was high, which is as long as the time the signal was out.
        // 1 timer count == 4 us so * 4 to get a value in microsecs
        // we divide by 58 to get the distance in cm, since (34000 cm/s * 1e-6 us time)/2 (back and forth measurement)
        // == 0.017 more or less 1/58
        let value = (timer.tcnt1.read().bits() * 4) / 58;
    
        let distancia = u16::from(value);
     }
}

use arduino_hal::prelude::*;
const TURNING_TIME: u16 = 700u16;

/// The mutable wheels array is destructured for easier manipulation.
pub fn go_forward(
    wheels: &mut [arduino_hal::port::Pin<arduino_hal::port::mode::Output>; 4],
) {
    // Be careful here with the order of unpacking. In my case, pin 4 is connected to left forward, 5 to left backwards, etc
    let [left_forw, left_back, right_forw, right_back] = wheels;
    left_forw.set_high();
    right_forw.set_high();

    left_back.set_low();
    right_back.set_low();
}

pub fn go_backward(
    wheels: &mut [arduino_hal::hal::port::Pin<arduino_hal::hal::port::mode::Output>; 4],
) {
    let [left_forw, left_back, right_forw, right_back] = wheels;

    left_forw.set_low();
    right_forw.set_low();

    left_back.set_high();
    right_back.set_high();
}

pub fn turn_right(
    wheels: &mut [arduino_hal::hal::port::Pin<arduino_hal::hal::port::mode::Output>; 4],
) {
    stop(wheels);
    let [left_forw, _, _, _] = wheels;

    let mut delay = arduino_hal::Delay::new();
    left_forw.set_high();
    delay.delay_ms(TURNING_TIME);
}
pub fn turn_left(
    wheels: &mut [arduino_hal::hal::port::Pin<arduino_hal::hal::port::mode::Output>; 4],
) {
    stop(wheels);
    let [_, _, right_forw, _] = wheels;

    let mut delay = arduino_hal::Delay::new();
    right_forw.set_high();
    delay.delay_ms(TURNING_TIME);
}

pub fn stop(wheels: &mut [arduino_hal::hal::port::Pin<arduino_hal::hal::port::mode::Output>; 4]) {
    let [left_forw, left_back, right_forw, right_back] = wheels;

    left_forw.set_low();
    left_back.set_low();
    right_forw.set_low();
    right_back.set_low();
}
