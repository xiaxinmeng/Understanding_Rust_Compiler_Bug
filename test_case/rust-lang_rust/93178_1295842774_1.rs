rs
let p: Pin<&mut _> = pin!(thing); // `thing` **moved out**
let r: Pin<&_> = p.as_ref();
