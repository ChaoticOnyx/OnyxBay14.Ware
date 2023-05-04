use crate::{PciDeviceFunc, PCI_FUNCS_COUNT};

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub(crate) address: usize,
    pub(crate) funcs: [PciDeviceFunc; PCI_FUNCS_COUNT as usize],
}

impl PciDevice {
    pub fn address(&self) -> usize {
        self.address
    }

    pub fn funcs(&self) -> &[PciDeviceFunc; PCI_FUNCS_COUNT as usize] {
        &self.funcs
    }
}
