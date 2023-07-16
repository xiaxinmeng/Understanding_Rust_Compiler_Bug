rs
fn a<F>(_f: F) where F: FnMut(&i32) {}

fn b<'a, F>(_f: F) where F: FnMut(&mut FnMut(&'a i32)) {}

fn main()
{
    b(|f| {
        a(|v| f(v))
    });
}
