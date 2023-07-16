rust
fn main() {
    let f = |x: &i32| x;
    let f = |x: &i32| -> &i32 { x };
    let f: impl for<'a> Fn(&'a i32) -> &'a i32 = |x| x;
    let f = for<'a> |x: &'a i32| -> &'a i32 { x };

    let i = &3;
    let j = f(i);
}
