
#![feature(float_to_from_bytes)]
pub fn compare() -> bool {
    let bytes = 12.5f32.to_ne_bytes();
    let arr = if cfg!(target_endian = "big") {
         [0x41, 0x48, 0x00, 0x00]
    } else {
        [0x00, 0x00, 0x48, 0x41]
    };

    bytes == arr
}
