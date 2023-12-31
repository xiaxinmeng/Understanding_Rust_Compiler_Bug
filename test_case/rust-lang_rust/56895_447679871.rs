rust
#![feature(self_struct_ctor)]

trait FooTrait {}

trait BarTrait {
    fn foo<T: FooTrait>(_: T) -> Self;
}

struct FooStruct { x: u32 }

impl BarTrait for FooStruct {
    fn foo<T: FooTrait>(_: T) -> Self {
        Self(0)
    }
}