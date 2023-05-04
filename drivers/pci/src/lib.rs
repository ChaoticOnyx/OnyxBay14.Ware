#![no_std]
#![no_main]

mod pci_device;
mod pci_device_bar;
mod pci_device_func;

use core::mem::MaybeUninit;

use drivers_mmio::Mmio;
pub use pci_device::PciDevice;
pub use pci_device_bar::PciDeviceBar;
pub use pci_device_func::{PciDeviceFunc, PciDeviceFuncInfo};

pub const PCI_FUNCS_COUNT: u8 = 8;
pub const PCI_FUNC_BARS_COUNT: u8 = 6;
pub const PCI_DEVICE_ID_REG: usize = 0x0;
pub const PCI_DEVICE_STATUS_REG: usize = 0x4;
pub const PCI_DEVICE_CLASS_REG: usize = 0x8;
pub const PCI_DEVICE_BAR0_REG: usize = 0x10;
pub const PCI_DEVICE_BAR1_REG: usize = 0x14;
pub const PCI_DEVICE_BAR2_REG: usize = 0x18;
pub const PCI_DEVICE_BAR3_REG: usize = 0x1C;
pub const PCI_DEVICE_BAR4_REG: usize = 0x20;
pub const PCI_DEVICE_BAR5_REG: usize = 0x24;
pub const PCI_DEVICE_IRQ_REG: usize = 0x3C;

#[derive(Debug, Clone, Copy)]
pub struct Pci {
    mmio: Mmio,
}

impl Pci {
    pub fn new(address: usize) -> Self {
        Self {
            mmio: Mmio::new(address),
        }
    }

    pub unsafe fn device(&self, device_id: u8) -> Option<PciDevice> {
        let base_offset = ((device_id as u32) << 15) as usize;
        let mut funcs: [MaybeUninit<PciDeviceFunc>; PCI_FUNCS_COUNT as usize] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (func_id, func) in funcs.iter_mut().enumerate() {
            let func_offset = (base_offset | func_id << 8) as usize;
            let ids_offset = func_offset | PCI_DEVICE_ID_REG;

            let ids = self.mmio.read_u32(ids_offset);

            if ids == 0xFFFFFFFF {
                return None;
            }

            let statuses_offset = func_offset | PCI_DEVICE_STATUS_REG;
            let statuses = self.mmio.read_u32(statuses_offset);

            let classes_offset = func_offset | PCI_DEVICE_CLASS_REG;
            let classes = self.mmio.read_u32(classes_offset);

            let irqs_offset = func_offset | PCI_DEVICE_IRQ_REG;
            let irqs = self.mmio.read_u32(irqs_offset);

            let mut bars: [Option<PciDeviceBar>; PCI_FUNC_BARS_COUNT as usize] =
                [None, None, None, None, None, None];

            for (bar_id, bar) in bars.iter_mut().enumerate() {
                let reg = PCI_DEVICE_BAR0_REG + bar_id * 4;
                let bar_address = self.mmio.read_u32(func_offset | reg) as usize;

                if bar_address != 0 {
                    *bar = Some(PciDeviceBar(Mmio::new(bar_address)))
                }
            }

            func.write(PciDeviceFunc {
                info: PciDeviceFuncInfo {
                    vendor_id: ids as u16,
                    device_id: (ids >> 16) as u16,
                    command: statuses as u16,
                    status: (statuses >> 16) as u16,
                    rev: classes as u8,
                    prog_if: (classes >> 8) as u8,
                    class_code: (classes >> 16) as u16,
                    irq_line: irqs as u8,
                    irq_pin: (irqs >> 8) as u8,
                },
                bars,
            });
        }

        let funcs = unsafe { core::mem::transmute(funcs) };

        Some(PciDevice {
            address: base_offset as usize,
            funcs,
        })
    }
}
