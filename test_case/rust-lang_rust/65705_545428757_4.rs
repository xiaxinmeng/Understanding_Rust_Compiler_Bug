rust
let mut v: Vec<Foobar<T>> = unimplemented!();
let values: Vec<T> = unsafe { v.transmute() };
