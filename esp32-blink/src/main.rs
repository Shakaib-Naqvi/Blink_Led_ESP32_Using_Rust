// // #![no_std]
// // #![no_main]

// // use esp_backtrace as _;
// // use esp_hal::{delay::Delay, prelude::*};

// // #[entry]
// // fn main() -> ! {
// //     #[allow(unused)]
// //     let peripherals = esp_hal::init(esp_hal::Config::default());
// //     let delay = Delay::new();

// //     esp_println::logger::init_logger_from_env();

// //     loop {
// //         log::info!("Hello world!");
// //         delay.delay(500.millis());
// //     }
// // }


// // #![no_std]
// // #![no_main]
// // #![feature(type_alias_impl_trait)]

// // use esp32::{clock::ClockControl, i2c::I2C, peripherals::Peripherals, prelude::*, IO};
// // use esp_backtrace as _;
// // use esp_println::println;

// #[entry]
// fn main() -> ! {
//     let peripherals = Peripherals::take();
//     let system = peripherals.SYSTEM.split();
//     let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

//     // Obtain handle for GPIO
//     let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

//     // Initialize and configure I2C0
//     let mut i2c0 = I2C::new(
//         peripherals.I2C0,
//         io.pins.gpio3,
//         io.pins.gpio2,
//         100u32.kHz(),
//         &clocks,
//     );

//     // This line is for Wokwi only so that the console output is formatted correctly
//     esp_println::print!("\x1b[20h");

//     // Start Scan at Address 1 going up to 127
//     for addr in 1..=127 {
//         println!("Scanning Address {}", addr as u8);

//         // Scan Address
//         let res = i2c0.read(addr as u8, &mut [0]);

//         // Check and Print Result
//         match res {
//             Ok(_) => println!("Device Found at Address {}", addr as u8),
//             Err(_) => println!("No Device Found"),
//         }
//     }

//     // Loop Forever
//     loop {}
// }



#![no_std]
#![no_main]

fn main(){
    
}