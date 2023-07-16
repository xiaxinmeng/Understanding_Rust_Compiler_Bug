rust
impl<const S: usize> ufmt::uDisplay for Buffer<S> {
    fn fmt<W: ufmt::uWrite + ?Sized>(
        &self,
        f: &mut ufmt::Formatter<'_, W>,
    ) -> Result<(), W::Error> {
        for ch in self.0.iter() {
            f.write_char(*ch as char);
        }

        Ok(())
    }
}
