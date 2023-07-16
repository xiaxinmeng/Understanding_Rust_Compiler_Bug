 rust
#![crate_type="lib"]

pub fn baud(wanted: u32) -> u32 {
    // baud is given by the following:
    // baud = 65536*(1-(samples_per_bit)*(f_wanted/f_ref))
    // samples_per_bit = 16, 8, or 3
    // f_ref = 48e6
    let wanted_f: f32 = wanted as f32;
    let baud: f32 = 65536f32 * (1f32 - (16f32 * (wanted_f / 48_000000f32)));
    baud.floor() as u32
}

pub fn baud2() -> u32 {
    baud(9600)
}
