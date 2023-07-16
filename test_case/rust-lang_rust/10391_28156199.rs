
fn foo<'a, T, Iter: Iterator<&'a T>>(_iter: Iter) { }

fn main() {
    let x = [1,2,3];
    foo(x.iter());
}
