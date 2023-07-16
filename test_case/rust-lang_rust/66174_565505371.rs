rust
        let mut buf: [MaybeUninit<u8>; N] = MaybeUninit::uninit_array();
        Initializer::zeroing().initialize(unsafe { buf.assume_init_mut() });
        let mut buf = unsafe { buf.assume_init() };
        