#![no_std]
#![no_main]
#![allow(dead_code)]

mod vga_text_mode;
use vga_text_mode as vga;

pub(crate) mod util;

#[no_mangle]
pub extern "C" fn kmain() {
    vga::clear_screen();
    vga::disable_cursor();
    vga::write_str("hello, world!");
    loop {}
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
