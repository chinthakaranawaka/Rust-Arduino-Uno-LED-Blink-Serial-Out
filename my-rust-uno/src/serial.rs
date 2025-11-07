#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // Initialize serial communication at 9600 baud
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    
    // Send startup message
    ufmt::uwriteln!(&mut serial, "Hello from Arduino Uno!\r").unwrap();
    
    let mut counter = 0u32;
    
    loop {
        // Send counter value
        ufmt::uwriteln!(&mut serial, "Counter: {}\r", counter).unwrap();
        
        // Simple delay
        arduino_hal::delay_ms(100);
        
        counter = counter.wrapping_add(1);
    }
}