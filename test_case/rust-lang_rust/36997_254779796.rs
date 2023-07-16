
fn main() { 
    let x = 1;
    let mut y = 2;
    let z = borrow(&x, &y);
    &mut y;
    z;
}
fn borrow<'a, 'b>(x : &'a i32, y : &'a i32) -> &'a i32 { &x }
