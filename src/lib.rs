#![no_std]

#[cfg(target_arch = "wasm32")]
use core::panic::PanicInfo;

use wasm_macro::custom_mangle;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[custom_mangle]
pub fn add(a: u16, b: u16) -> u16 {
    a + b
}

#[custom_mangle]
pub fn mul(a: u16, b: u16) -> u32 {
    (a * b) as u32
}
