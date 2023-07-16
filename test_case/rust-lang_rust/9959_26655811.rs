
let x: *T;
unsafe {
let mut a: &'static T = &*x;
}
