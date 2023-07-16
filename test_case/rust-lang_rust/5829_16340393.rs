 rust
struct T<'self> {
    x: &'self mut int,
    y: int,
}

fn F1<'a>(t: &'a mut T<'a>) {
    *&mut t.x = &mut t.y;
}

fn F2<'a, 'b>(_t: &'a mut T<'b>) {
}

fn main() {
    let mut x = 5;
    let mut t = T{x: &mut x, y: 6};
    F2(&mut t);
    F2(&mut t);
}
