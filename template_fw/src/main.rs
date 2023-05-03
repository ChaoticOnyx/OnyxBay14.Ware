#![no_std]
#![no_main]
#![feature(panic_info_message, strict_provenance)]

use core::arch::global_asm;

global_asm!(include_str!("asm.S"));

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start_rust() {
    loop {}
}

#[no_mangle]
extern "C" fn _start_trap() {
    loop {}
}
