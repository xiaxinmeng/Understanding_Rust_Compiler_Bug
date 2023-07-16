rust
let vec: Vec<T> = ...;
// so that dropping the vec doesn't deallocate 
let mut vec = ManuallyDrop::new(vec);
// get each part
let len = vec.len();
let cap = vec.cap();
let ptr = vec.as_mut_ptr();
// make the wrapped vec unusable
drop(vec);
