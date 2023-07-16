rust
impl<'a> Adc<'a> {
    pub fn is_conv_done(&mut self) -> bool {
        // self.reg has type AdcRegs which is packed; then we call RW::read(&self, u32)
        unsafe { self.reg.sc1a.read().get_bit(7) }
  }
}
