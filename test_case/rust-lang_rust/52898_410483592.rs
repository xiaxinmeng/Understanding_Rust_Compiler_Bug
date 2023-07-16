
let mut x: &'static u8 = mem::zeroed();
x = &0;
do_a_thing(&x);
