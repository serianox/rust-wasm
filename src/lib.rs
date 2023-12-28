#![no_std]

use wasm_macro::custom_mangle;

#[link(wasm_import_module = "mod")]
extern {
    #[link_name = "some_other_name"]
    fn register();
}

#[link_section = "hello"]
#[no_mangle]
#[used]
pub static SECTION: [u8; 24] = *b"This is a custom section";

#[cfg(target_arch = "wasm32")]
#[panic_handler]
#[inline(always)]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[custom_mangle]
pub fn install(_parameters: u16) -> () {
    unsafe { register(); }
}

#[custom_mangle]
pub fn process(_a: u16, _b: u16) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[no_mangle]
pub fn mul(a: u16, b: u16) -> u32 {
    (a * b) as u32
}
