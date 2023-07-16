rust
struct S;
impl S {
    fn f<'a:'a>(_x:&'a i32) {}
}
fn main() {
    let g1: &Fn(&mut i32) = &|x| S::f(x);
    g1(&mut 1);
    let g2 = &|x| S::f(x);
    g2(&mut 1);
}
