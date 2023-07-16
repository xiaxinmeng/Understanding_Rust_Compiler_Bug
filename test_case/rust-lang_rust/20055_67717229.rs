 rust
pub fn main() {
    let _: Box<[int]> = if true { box [1i, 2, 3] } else { box [1i] };

    let _: Box<[int]> = match true { true => box [1i, 2, 3], false => box [1i] };

    // Check we don't get over-keen at propagating coercions in the case of casts.
    let x = if true { 42 } else { 42u8 } as u16;
    let x = match true { true => 42, false => 42u8 } as u16;
}
