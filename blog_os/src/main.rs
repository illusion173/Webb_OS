#![no_std] // no link to rust std
#![no_main] // disable rust level entry pnt
mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::Writer.lock().write_str("Hello").unwrap();

    loop {}
}
