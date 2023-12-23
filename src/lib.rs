#![no_std]

#[cfg(target_arch="wasm32")]
use core::panic::PanicInfo;

#[cfg(target_arch="wasm32")]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

//#[no_mangle]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
