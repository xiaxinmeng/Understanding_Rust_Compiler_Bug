rust
#![feature(ptr_metadata)]

fn bar<U: 'static + ?Sized>() -> <U as core::ptr::Pointee>::Metadata {panic!()}
