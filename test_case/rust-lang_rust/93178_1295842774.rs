rs
let p: &mut _ = Box::leak(thing); // `thing` **moved out**
let r: &_ = &*p;
