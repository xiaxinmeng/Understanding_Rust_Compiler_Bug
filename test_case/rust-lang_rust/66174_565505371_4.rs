rust
unsafe trait ReadIntoUninit : Read {
    fn read_into_uninit<'buf> (self: &'_ mut Self, buf: impl Into<OutSlice<'buf, u8>> + 'buf)
      -> Result<&'buf mut [u8]>
    ;
    ...
}

unsafe impl<R : ?Sized + Read> ReadIntoUninit for R {
    default
    fn read_into_uninit<'buf> (self: &'_ mut Self, buf: impl Into<OutSlice<'buf, u8>> + 'buf)
      -> Result<&'buf mut [u8]>
    {
        let mut buf = buf.into();
        let buf: &mut [u8] = buf.write_all(&0); // Initializer::zeroing().initialize(&mut buf) here
        self.read(buf).map(|n| &mut buf[.. n])
    }
}
