
error[E0277]: `PhantomPinned` cannot be unpinned
   --> src/main.rs:5:59
    |
5   |     let val: Pin<Pin<&PhantomPinned>> = Pin::new(Pin::new(&PhantomPinned));
    |                                                           ^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `PhantomPinned`
    |
    = note: consider using `Box::pin`
note: required by `Pin::<P>::new`

error[E0277]: `PhantomPinned` cannot be unpinned
   --> src/main.rs:5:50
    |
5   |     let val: Pin<Pin<&PhantomPinned>> = Pin::new(Pin::new(&PhantomPinned));
    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `PhantomPinned`
    |
    = note: consider using `Box::pin`
note: required by `Pin::<P>::new`
