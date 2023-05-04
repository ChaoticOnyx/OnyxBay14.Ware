use drivers_mmio::Mmio;

#[derive(Debug, Clone, Copy)]
pub struct PciDeviceBar(pub Mmio);
