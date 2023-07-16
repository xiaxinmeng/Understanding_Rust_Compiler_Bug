 rust
pub trait Memory {
    fn read_u8(&self, addr: u16) -> u8;
    fn write_u8(&mut self, addr: u16, val: u8);
}
