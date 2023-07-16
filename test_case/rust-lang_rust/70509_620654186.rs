rust
impl ::core::cmp::PartialEq for Test {
    #[inline]
    fn eq(&self, other: &Test) -> bool {
        {
            let __self_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*self) } as
                    i128;
            let __arg_1_vi =
                unsafe { ::core::intrinsics::discriminant_value(&*other) } as
                    i128;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Test::A(ref __self_0), &Test::A(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Test::B(ref __self_0), &Test::B(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
