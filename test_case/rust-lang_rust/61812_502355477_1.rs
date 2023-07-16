rust
trait Tr {
    const C: u8 = 255u8 + 1;
}

impl Tr for () {
    const C: u8 = 255u8 + 1;
}
