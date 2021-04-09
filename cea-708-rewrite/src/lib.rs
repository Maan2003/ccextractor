#![no_std]
#![no_builtins]

pub mod extern_;

mod impl_;

use core::panic::PanicInfo;

#[panic_handler]
fn my_handler(_info: &PanicInfo) -> ! {
    loop {}
}

pub use extern_::*;
