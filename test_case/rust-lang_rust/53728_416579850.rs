rust
#[repr(u16)]
enum DeviceKind {
    Nil = 0,
}

#[repr(packed)]
struct DeviceInfo {
    endianness: u8,
    device_kind: DeviceKind,
}

fn main() {
    None::<(DeviceInfo, Box<u8>)>;
}
