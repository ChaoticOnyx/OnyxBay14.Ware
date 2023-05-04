#![no_std]
#![no_main]

use core::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Mmio {
    address: usize,
}

impl Mmio {
    pub const fn new(address: usize) -> Self {
        Self { address }
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub unsafe fn write_u8(&mut self, value: u8, offset: usize) {
        let ptr = (self.address + offset) as *mut u8;

        ptr.write_volatile(value);
    }

    pub unsafe fn write_u32(&mut self, value: u32, offset: usize) {
        let ptr = (self.address + offset) as *mut u32;

        ptr.write_volatile(value);
    }

    pub unsafe fn read_u8(&self, offset: usize) -> u8 {
        let ptr = (self.address + offset) as *const u8;

        ptr.read_volatile()
    }

    pub unsafe fn read_u16(&self, offset: usize) -> u16 {
        let ptr = (self.address + offset) as *const u16;

        ptr.read_volatile()
    }

    pub unsafe fn read_u32(&self, offset: usize) -> u32 {
        let ptr = (self.address + offset) as *const u32;

        ptr.read_volatile()
    }
}

impl Debug for Mmio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mmio")
            .field("address", &format_args!("{:#016X}", self.address))
            .finish()
    }
}
