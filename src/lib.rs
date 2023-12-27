#![no_std]

use wasm_macro::custom_mangle;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[cfg(target_arch = "wasm32")]
//#[panic_handler]
#[allow(dead_code)]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    alert("");
    unsafe { core::hint::unreachable_unchecked() }
}

#[custom_mangle]
pub fn install(_parameters: u16) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[custom_mangle]
pub fn process(_a: u16, _b: u16) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
