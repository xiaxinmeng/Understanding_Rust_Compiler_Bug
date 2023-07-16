rust
        let mut buf: [mem::MaybeUninit<u8>; super::DEFAULT_BUF_SIZE] = [mem::MaybeUninit::uninit(); super::DEFAULT_BUF_SIZE];
        let slice = slice::from_raw_parts_mut(MaybeUninit::first_ptr_mut(&mut buf), super::DEFAULT_BUF_SIZE);
        reader.initializer().initialize(slice);
