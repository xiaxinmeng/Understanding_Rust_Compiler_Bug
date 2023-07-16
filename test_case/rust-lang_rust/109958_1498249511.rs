rust
// CHECK-LABEL: @check_to_enum(
#[no_mangle]
pub unsafe fn check_to_enum(x: i8) -> SmallEnum {
    transmute(x)
}

// CHECK-LABEL: @check_from_enum(
#[no_mangle]
pub unsafe fn check_from_enum(x: SmallEnum) -> i8 {
    transmute(x)
}
