rust
type InterruptHandler = fn();

#[repr(C, packed)]
struct Descriptor {
    base_low: u16,
    selector: u16,
    flags: u16,
    base_high:u16,

impl Descriptor
    const fn new(handler: &InterruptHandler, selector: u16, flags: u16) -> Descriptor {
        Descriptor {
            base_low: (handler as *const _ as usize & 0xFFFF) as u16,
            selector: selector,
            flags: flags,
            base_high: ((handler as *const _ as usize >> 16) & 0xFFFF) as u16,
        }
    }
}
