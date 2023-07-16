
#![allow(warnings)]

use std::cell::Cell;

struct S1<'c, 'a: 'c, 'x: 'a>(&'c Cell<&'a u32>, &'x ());
struct S2<'c, 'b: 'c + 'x, 'x>(&'c Cell<&'b u32>, &'x ());

fn assert_sig<'a, 'b, 'c, F>(
    a: &'c Cell<&'a u32>,
    b: &'c Cell<&'b u32>,
    f: F,
) -> Box<for<'x> Fn(&'a &'x (), &'x &'b ()) + 'c>
where
    'a: 'c,
    'b: 'c,
    F: for<'x> Fn(S1<'c, 'a, 'x>, S2<'c, 'b, 'x>) + 'static,
{
    Box::new(move |x, y| f(S1(a, x), S2(b, y)))
}

fn manual_closure<'c, 'a, 'b, 'x>(x1: S1<'c, 'a, 'x>, x2: S2<'c, 'b, 'x>) {
    x1.0.set(x2.0.get());
}

fn main() {
    let (a, mut b, w0, w1, c_a);
    a = 0;
    b = 0;
    w0 = &();
    w1 = &();
    c_a = Cell::new(&a);
    let c_b = Cell::new(&b);

    // weirdly enough, this does *not* force 'a: 'b, because within
    // the closure, we know that 'a: 'x & 'x: 'b and therefore
    // 'a: 'b.
    let assert = assert_sig(&c_a, &c_b, manual_closure);

    if true {
        assert(&w0, &w1); // this forces 'a: 'x, 'x: 'b
    } else {
        // but *nothing* is forced here
        drop(assert); // <-- added this
        b = 1;
        println!("{}", c_a.get());
    }
}
