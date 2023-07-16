
let __self0_vi = unsafe {
    std::intrinsics::discriminant_value(&self) } as i32;
let __self1_vi = unsafe {
    std::intrinsics::discriminant_value(&__arg1) } as i32;
let __self2_vi = unsafe {
    std::intrinsics::discriminant_value(&__arg2) } as i32;
...

if __self0_vi == __self1_vi && __self0_vi == __self2_vi && ... {
    match (...) {
        (Variant1, Variant1, ...) => Body1
        (Variant2, Variant2, ...) => Body2,
        ...
        _ => ::core::intrinsics::unreachable()
    }
}
else {
    <delegated expression referring to __self0_vi, et al.>
}

