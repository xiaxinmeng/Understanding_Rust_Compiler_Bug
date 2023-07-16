Rust
    struct VecWrapper<'a>(&'a mut S);

    struct S(Box<u32>);

    fn get_dangling<'a>(v: VecWrapper<'a>) -> &'a u32 {
        let s_inner: &'a S = &*v.0; //~ ERROR
        &s_inner.0
    }

    impl<'a> Drop for VecWrapper<'a> {
        fn drop(&mut self) {
            *self.0 = S(Box::new(0));
        }
    }
    