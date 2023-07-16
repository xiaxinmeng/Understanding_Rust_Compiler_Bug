rust
#[repr(transparent)]
pub struct Foo(usize, core::mem::ManuallyDrop<()>);

#[repr(transparent)]
pub struct Bar(usize, std::cell::UnsafeCell<()>);

#[repr(transparent)]
pub struct Baz(usize, core::mem::MaybeUninit<()>);
