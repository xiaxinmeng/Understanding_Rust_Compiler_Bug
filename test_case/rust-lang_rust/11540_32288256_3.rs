
fn main() {
    struct T<'a> {i: int, j: &'a mut int}
    let mut x = 1;
    let y = T{i: 0, j: &mut x};
    fn foo(T{i: a, j: b}: T) {*b += a;}
    foo(T{ j: &mut *y.j, ..y});
    foo(T{ j: &mut *y.j, ..y});
}
