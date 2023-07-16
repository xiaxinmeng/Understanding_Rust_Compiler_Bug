
let mut y = 1;
let x: Box<&mut i8> = Box::new(&mut y);
**x = 2;
