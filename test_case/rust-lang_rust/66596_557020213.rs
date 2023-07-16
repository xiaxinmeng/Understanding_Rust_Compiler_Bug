rust
#![feature(const_generics)]

trait FieldType<const NAME: &'static str> {}

impl FieldType<"0"> for i32 {}

fn main() {
    let _: <i32 as FieldType<"0">>::FieldType;
}
