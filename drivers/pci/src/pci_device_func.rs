use core::fmt::Debug;

use crate::{PciDeviceBar, PCI_FUNC_BARS_COUNT};

#[derive(Debug, Clone)]
pub struct PciDeviceFunc {
    pub(crate) info: PciDeviceFuncInfo,
    pub(crate) bars: [Option<PciDeviceBar>; PCI_FUNC_BARS_COUNT as usize],
}

#[derive(Clone)]
pub struct PciDeviceFuncInfo {
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u16,
    pub prog_if: u8,
    pub rev: u8,
    pub irq_line: u8,
    pub irq_pin: u8,
    pub status: u16,
    pub command: u16,
}

impl PciDeviceFunc {
    pub fn info(&self) -> &PciDeviceFuncInfo {
        &self.info
    }

    pub fn bars(&self) -> &[Option<PciDeviceBar>; PCI_FUNC_BARS_COUNT as usize] {
        &self.bars
    }
}

impl Debug for PciDeviceFuncInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PciDeviceFunc")
            .field("vendor_id", &format_args!("{:#016X}", &self.vendor_id))
            .field("device_id", &format_args!("{:#016X}", self.device_id))
            .field("class_code", &format_args!("{:#016X}", self.class_code))
            .field("prog_if", &format_args!("{:#08X}", self.prog_if))
            .field("rev", &format_args!("{:#08X}", self.rev))
            .field("irq_line", &format_args!("{:#08X}", self.irq_line))
            .field("irq_pin", &format_args!("{:#08X}", self.irq_pin))
            .field("status", &format_args!("{:#016X}", self.status))
            .field("command", &format_args!("{:#016X}", self.command))
            .finish()
    }
}
