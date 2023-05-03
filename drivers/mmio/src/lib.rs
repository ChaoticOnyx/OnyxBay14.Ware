#![no_std]
#![no_main]

#[derive(Clone, Copy)]
pub struct Mmio {
    pub address: usize,
}

impl Mmio {
    pub const fn new(address: usize) -> Self {
        Self { address }
    }

    pub fn write_u8(&mut self, value: u8, offset: usize) {
        let ptr = (self.address + offset) as *mut u8;

        unsafe { ptr.write_volatile(value) };
    }

    pub fn write_u32(&mut self, value: u32, offset: usize) {
        let ptr = (self.address + offset) as *mut u32;

        unsafe { ptr.write_volatile(value) };
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
        let ptr = (self.address + offset) as *const u8;

        unsafe { ptr.read_volatile() }
    }

    pub fn read_u16(&self, offset: usize) -> u16 {
        let ptr = (self.address + offset) as *const u16;

        unsafe { ptr.read_volatile() }
    }

    pub fn read_u32(&self, offset: usize) -> u32 {
        let ptr = (self.address + offset) as *const u32;

        unsafe { ptr.read_volatile() }
    }
}
