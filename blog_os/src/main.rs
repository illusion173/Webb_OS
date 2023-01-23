#![no_std] // no link to rust std
#![no_main] // disable rust level entry pnt
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
use core::panic::PanicInfo;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial assertion!");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    /*
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    */
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello world {}", "!");
    //panic!("AHHH");
    //
    #[cfg(test)]
    test_main();
    loop {}
}
