#![no_std]
#![no_main]


#[no_mangle]
pub extern "C" fn kmain() {
    // eventually write hello world to screen
    loop {}
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
