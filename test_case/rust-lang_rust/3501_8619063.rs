
struct A {
    x: ~int,
    mut y: ~[&~int],
}

fn push<T>(v: &mut ~[T], +x: T) {
    vec::push(*v, move x);
}

fn A() -> A {
    let a = A { x: ~1, y: ~[] };
    push(&mut a.y, &a.x);
    a
}

fn main() {
    let a = A();
    error!("%?", a);
}
