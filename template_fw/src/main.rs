// Read more https://github.com/rust-embedded/riscv-rt/blob/master/src/lib.rs

#![no_std]
#![no_main]

use riscv_rt::entry;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    loop {}
}
