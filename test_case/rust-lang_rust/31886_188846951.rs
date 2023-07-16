 rust
enum Eu64 { Au64 = 0, Bu64 = 9223372036854775808, }

impl ::std::cmp::PartialOrd for Eu64 {
    #[inline]
    fn partial_cmp(&self, __arg_0: &Eu64)
     -> ::std::option::Option<::std::cmp::Ordering> {
        /* elided */
    }
    #[inline]
    fn lt(&self, __arg_0: &Eu64) -> bool {
        /* elided */
    }
    #[inline]
    fn le(&self, __arg_0: &Eu64) -> bool {
        /* elided */
    }
    #[inline]
    fn gt(&self, __arg_0: &Eu64) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    i32;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*__arg_0) } as
                    i32;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*__arg_0) {
                    (&Eu64::Au64, &Eu64::Au64) => false,
                    (&Eu64::Bu64, &Eu64::Bu64) => false,
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { __self_vi.gt(&__arg_1_vi) }
        }
    }
    #[inline]
    fn ge(&self, __arg_0: &Eu64) -> bool {
        /* elided */
    }
}

impl ::std::cmp::PartialEq for Eu64 {
    /* elided */
}
