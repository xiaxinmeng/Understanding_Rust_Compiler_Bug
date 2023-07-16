
let mut ptr = NonNull::new(...).unwrap(); // the mut is required
unsafe {
    *ptr.as_mut() = ...; // as_mut is a mutable borrow of ptr
}

let mut ptr = NonNull::new(...).unwrap(); // the mut is unnecessary and will trigger a lint
unsafe {
    *ptr.as_uninit_mut().write(...); // no borrow of ptr
}
