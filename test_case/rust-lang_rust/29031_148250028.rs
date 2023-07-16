 rust
let (a, b) = {
    let Mutex { ref a, ref b } = self; // note exhaustive match
    (ptr::read(a), ptr::read(b))
};
mem::forget(self);
