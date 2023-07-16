rust
#![feature(ptr_metadata)]

trait HasType {
    type Type;
}

impl HasType for () {
    type Type = ();
}

pub struct MyStruct {
    _field: <() as HasType>::Type,
}

pub fn bar() {
    let val: <MyStruct as std::ptr::Pointee>::Metadata;
}
