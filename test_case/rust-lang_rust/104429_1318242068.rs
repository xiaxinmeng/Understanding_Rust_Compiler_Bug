
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightControlState {
    Stored = 0x00,
    Custom = 0x01,
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LightControlWriteCheck {
    Ok = 0x01,
    Invalid = 0x02,
    Busy = 0x03,
}

//#[derive(HidrawRead, HidrawWrite, Debug)]
#[derive(Debug)]
#[repr(C, packed)]
pub struct LightControl {
    //#[hidraw_constant = "0x13"]
    _report_id: u8,
    //#[hidraw_constant = "::std::mem::size_of::<Self> as u8"]
    _size: u8,
    pub state: LightControlState,
    pub unknown0: [u8; 3],
    pub write_check: LightControlWriteCheck,
    pub unknown1: u8,
}
