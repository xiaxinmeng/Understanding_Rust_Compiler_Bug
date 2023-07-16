
struct A<'a> { b: &'a mut int }

fn foo(a: &mut A) {
    *a.b = 3;
}

fn main() {
    let mut a = 2;
    let mut b = A { b: &mut a };
    foo(&mut b);
}
