rust
struct SomeStruct(PhantomPinned);
let pinned = Rc::pin(SomeStruct(PhantomPinned));

// This is unsound !!!
let weak = Rc::as_weak(pinned.as_ref());
// ... because it would be possible to move the content of pinned:
let mut unpinned_rc = weak.upgrade().unwrap();
std::mem::drop(pinned);
// unpinned_rc is now the only reference so this will work:
let x = std::mem::replace(
    Rc::get_mut(&mut unpinned_rc).unwrap(),
    SomeStruct(PhantomPinned),
);
