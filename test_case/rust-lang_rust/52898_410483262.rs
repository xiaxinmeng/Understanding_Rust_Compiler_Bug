
let mut x: u8 = mem::uninitialized();
x = 0; // not uninitialized anymore, and therefore not really undefined behavior
do_a_thing(&x);
