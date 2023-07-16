
struct Foo { f: int }

fn a(p: &v/mut Foo) -> &v/mut int { &mut p.f }

fn b(p: &v/mut Foo) -> &v/mut int {
    let q = a(&mut *p);
    p.f += 1; //~ ERROR p should be lent out here
    return q;
}

fn c() {
    let mut f = Foo { f: 3 };
    let b = b(&mut f);
    f.f += 1; //~ ERROR, f is lent out
    *b += 1; // OK
}

fn main() {}
