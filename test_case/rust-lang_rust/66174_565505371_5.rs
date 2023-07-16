rust
    unsafe impl ReadIntoUninit for MyType {
        fn read_into_uninit<'buf> (self:&'_ mut Self, buf: impl Into<OutSlice<'buf, u8>> + 'buf)
          -> Result<&'buf mut [u8]>
        {
            let mut buf = buf.into();
            // implementaiton goes here
        }
    }

    impl Read for MyType {
        fn read (self: &'_ mut Self, buf: &'_ mut [u8])
          -> Result<usize>
        {
            self.read_into_uninit(buf).map(|slice| slice.len())
        }
    }
    