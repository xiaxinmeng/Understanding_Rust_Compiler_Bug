rust
trait MyTrait { }
impl<'a> MyTrait for &'a i32 { }

fn example<R: MyTrait, F: Fn(&i32) -> R>(_f: F) { }

fn main() {
    let func: fn(&i32) -> &i32 = |x| x;
    example(func);
}
