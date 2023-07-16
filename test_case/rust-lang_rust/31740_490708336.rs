#[inline]
fn eq(&self, other: &MyEnum) -> bool {
    {
        let __self_vi =
            unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                isize;
        let __arg_1_vi =
            unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                isize;
        if true && __self_vi == __arg_1_vi {
            match (&*self, &*other) {
                (&MyEnum::B(ref __self_0), &MyEnum::B(ref __arg_1_0)) =>
                (*__self_0) == (*__arg_1_0),
                _ => true,
            }
        } else { false }
    }
}