#![no_std]

#[cfg(target_arch="wasm32")]
use core::panic::PanicInfo;

use wasm_macro::return_as_is;

#[cfg(target_arch="wasm32")]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[return_as_is]
#[export_name = "00"]
pub fn add(a: u16, b: u16) -> u16 {
    a + b
}

#[return_as_is]
#[export_name = "01"]
pub fn mul(a: u16, b: u16) -> u32 {
    (a * b) as u32
}
