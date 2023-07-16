rust
#[automatically_derived]
#[allow(unused_qualifications)]
#[stable(feature = "rust1", since = "1.0.0")]
impl <T: ::std::cmp::PartialEq> ::std::cmp::PartialEq for Option<T> {
    #[inline]
    fn eq(&self, __arg_0: &Option<T>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*__arg_0) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*__arg_0) {
                    (&Option::Some(ref __self_0),
                     &Option::Some(ref __arg_1_0)) =>
                    true && (*__self_0) == (*__arg_1_0),
                    _ => true,
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &Option<T>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*__arg_0) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*__arg_0) {
                    (&Option::Some(ref __self_0),
                     &Option::Some(ref __arg_1_0)) =>
                    false || (*__self_0) != (*__arg_1_0),
                    _ => false,
                }
            } else { true }
        }
    }
}
