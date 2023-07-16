 Rust
trait Trait<'root> {}
impl<'a,'root> Trait<'root> for &'a i32 where 'a: 'root {}
fn foo<'root, T:Trait<'root>>(t: T) -> i32 {
    /* ... - your original signature doesn't specify the return type's lifetime */
}

fn main() {
    'v: {
        let v = &2;
        ('t: { foo::<'t, &'v i32> })(&v);
    }
}
