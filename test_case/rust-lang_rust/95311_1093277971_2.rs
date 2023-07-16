rust
#![feature(ptr_metadata)]

trait Foo {
    fn bar<U: 'static + ?Sized>() -> <U as core::ptr::Pointee>::Metadata { panic!() }
}
