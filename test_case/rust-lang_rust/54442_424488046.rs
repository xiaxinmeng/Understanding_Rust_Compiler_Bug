
fn low7<T: Copy + CastInto<u8>>(value: T) -> u8 {
  value as u8 & 0b01111111
}
