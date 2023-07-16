
        // &[u8] and &[T::ULE] are the same slice with different length metadata.
        let data = bytes.as_ptr();
        let mut metadata = bytes.len();
        metadata /= core::mem::size_of::<T::ULE>();
        Self::Borrowed(core::mem::transmute((data, metadata)))
