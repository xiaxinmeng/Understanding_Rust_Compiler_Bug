rust
fn main() { unsafe {
// create two smart pointers to a T
let mut rc0 = Rc::new(Box::new(0_i32));
let ptr: *mut Box<i32> = Rc::get_mut(&mut rc0).unwrap();
let rc1 = rc0.clone();
// drop the T
std::ptr::drop_in_place(ptr);
// those two smart pointers still exist
dbg!(Rc::strong_count(&rc0)); // prints 2
// they also compare equal
assert!(Rc::ptr_eq(&rc0, &rc1)); // ALWAYS OK
// put a new T behind the Rc
ptr.write(Box::new(42));
}}
