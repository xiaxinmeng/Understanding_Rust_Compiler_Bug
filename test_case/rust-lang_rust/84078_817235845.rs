rust
enum MyEnum {
    A,
    // B,
    C,
}

fn to_int(my_enum: MyEnum) -> i32 {
    use MyEnum::{A, B, C};  //~ error[E0432]: unresolved import `MyEnum::B`
    match my_enum {
        A => 1,
        B => 2,
        C => 3,
    }
}
