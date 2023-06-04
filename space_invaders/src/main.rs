// Read more https://github.com/rust-embedded/riscv-rt/blob/master/src/lib.rs

#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate alloc;

mod bsod;
mod game;
mod image;
mod io;
mod stack_string;
mod text;
mod time;
mod video;

use core::fmt::Write;

use bsod::bsod;
use game::Game;
use heap::Heap;
use riscv::register::{
    mcause::{Exception, Trap},
    sstatus::FS,
};
use riscv_rt::entry;

pub use image::Image;
pub use io::Io;
pub use stack_string::StackString;
pub use text::Text;
pub use time::Time;
pub use video::Video;

extern "C" {
    static _sheap: u8;
    static _heap_size: u8;
}

#[global_allocator]
pub static ALLOCATOR: Heap = Heap::empty();

unsafe fn init_heap() {
    let start = &_sheap as *const u8 as usize;
    let size = &_heap_size as *const u8 as usize;

    ALLOCATOR.init(start as *mut u8, size)
}

#[panic_handler]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(reason) = info.payload().downcast_ref::<&str>() {
        bsod(Some(reason), info.location());
    } else if let Some(message) = info.message() {
        let mut msg = StackString::new();

        write!(&mut msg, "PANIC: {message}").unwrap();

        bsod(Some(msg.str()), info.location());
    } else {
        bsod(Some("PANIC"), info.location());
    }
}

#[entry]
unsafe fn main() -> ! {
    riscv::register::mstatus::set_fs(FS::Initial);

    Video::default().init();

    init_heap();

    Io::default().init();

    Game::default().start();

    loop {
        riscv::asm::wfi();
    }
}

#[export_name = "ExceptionHandler"]
pub fn exception(_trap_frame: &riscv_rt::TrapFrame) -> ! {
    let Trap::Exception(exception) = riscv::register::mcause::read().cause() else {
        unreachable!()
    };

    match exception {
        Exception::InstructionMisaligned => bsod(Some("Instruction misaligned"), None),
        Exception::InstructionFault => bsod(Some("Instruction fault"), None),
        Exception::IllegalInstruction => bsod(Some("Illegal instruction"), None),
        Exception::Breakpoint => bsod(Some("Breakpoint"), None),
        Exception::LoadMisaligned => bsod(Some("Load misaligned"), None),
        Exception::LoadFault => bsod(Some("Load fault"), None),
        Exception::StoreMisaligned => bsod(Some("Store misaligned"), None),
        Exception::StoreFault => bsod(Some("Store fault"), None),
        Exception::UserEnvCall => bsod(Some("User EnvCall"), None),
        Exception::SupervisorEnvCall => bsod(Some("Supervisor EnvCall"), None),
        Exception::MachineEnvCall => bsod(Some("Machine EnvCall"), None),
        Exception::InstructionPageFault => bsod(Some("Instruction page fault"), None),
        Exception::LoadPageFault => bsod(Some("Load page fault"), None),
        Exception::StorePageFault => bsod(Some("Store page fault"), None),
        Exception::Unknown => bsod(Some("Unknown"), None),
    }
}
