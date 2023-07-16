rust
use std::pin::Pin;
use std::marker::PhantomData;
use std::marker::PhantomPinned;

struct Thing(
    String,
    PhantomData<PhantomPinned>, // Comment out this line and there's no error
);

// Uncomment this line and there's no error
// impl Unpin for Thing {}

impl Thing {
    fn foo<'a>(mut p: Pin<&'a mut Thing>) {
        // The error this produces is difficult to understand.
        // warning: variable does not need to be mutable
        // error: cannot borrow data in a `&` reference as mutable
        let _q: Pin<&mut String> = Pin::new(&mut p.0);
    }
}
