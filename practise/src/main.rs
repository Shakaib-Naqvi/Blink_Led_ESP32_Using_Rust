#![no_std]
#![no_main]

use esp32_hal::entry;
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
