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
    vga_buffer::print_something();
       loop {}
}
