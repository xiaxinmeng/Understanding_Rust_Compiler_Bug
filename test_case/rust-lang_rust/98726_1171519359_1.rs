rs
fn a<F>(_f: F) where F: FnMut(&i32) {}

fn b<F>(_f: F) where F: FnMut(&mut for<'a> FnMut(&'a i32)) {}

fn main()
{
    b(|f| {
        a(|v| f(v))
    });
}
