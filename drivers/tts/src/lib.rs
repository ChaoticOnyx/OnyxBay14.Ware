#![no_std]
#![no_main]

use drivers_pci::{PciDevice, PciDeviceBar};

#[derive(Debug, Clone)]
pub struct Tts {
    device: PciDevice,
}

impl Tts {
    fn bar_mut(&mut self) -> PciDeviceBar {
        self.device.funcs()[0].bars()[0].unwrap()
    }

    fn bar(&self) -> PciDeviceBar {
        self.device.funcs()[0].bars()[0].unwrap()
    }

    pub unsafe fn write_string(&mut self, text: &str) {
        let PciDeviceBar(mut mmio) = self.bar_mut();

        for ch in text.as_bytes() {
            mmio.write_u8(*ch, 0x0)
        }
    }

    pub unsafe fn string_length(&self) -> u32 {
        let PciDeviceBar(mmio) = self.bar();

        mmio.read_u32(0x0)
    }

    pub unsafe fn flush(&mut self) {
        let PciDeviceBar(mut mmio) = self.bar_mut();

        mmio.write_u8(0, 0x2);
    }

    pub unsafe fn speech(&mut self) {
        let PciDeviceBar(mut mmio) = self.bar_mut();

        mmio.write_u8(0, 0x1);
    }
}

impl From<PciDevice> for Tts {
    fn from(value: PciDevice) -> Self {
        Self { device: value }
    }
}
