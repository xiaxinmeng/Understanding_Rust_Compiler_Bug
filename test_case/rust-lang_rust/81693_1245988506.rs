rust
struct A<'a>(&'a i32);

impl<'a> Drop for A<'a> {
    fn drop (&mut self)
    {}
}

fn main ()
{
    let mut x = 10;
    let r = (A(&x), );
    let a = r.0;
    drop(a);
    x = 11; // Ok if A wasn't Drop
}
