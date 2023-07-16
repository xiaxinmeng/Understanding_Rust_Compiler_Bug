
fn main() { 
    let x = 1;
    let mut y = 2;
    let z = borrow(&x, &y);
    &mut y;
    z;
}
fn borrow<'a, 'b>(u : &'a i32, v : &'a i32) -> &'a i32 { &u }
