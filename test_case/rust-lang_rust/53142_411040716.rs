rust
struct X<'a, T: 'a> {
    a: &'a T,
    b: isize,
    c: isize,
}

struct Y<'a, T: 'a> {
    a: &'a T,
    b: usize,
}

fn new_y<'a, T: 'a>(x: &'a T) -> Y<'a, T> {
    Y {
        a: x,
        b: 100,
    }
}

fn main() {
    let a = &200isize;
    let y = new_y(&X{a: a, b: 300, c:400});
    let f = || { println!("{}", y.a.b); };
    (|| { f() })();
}
