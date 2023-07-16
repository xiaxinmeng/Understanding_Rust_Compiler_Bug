rust
trait Object {
    type Output;
}

impl<T: ?Sized> Object for T {
    type Output = &'static u64;
}

fn foo<'a, T: ?Sized>(x: <T as Object>::Output) -> &'a u64 {
    x
}

fn transmute_lifetime<'a, 'b>(x: &'a u64) -> &'b u64 {
    foo::<dyn Object<Output=&'a u64>>(x)
}

// And yes this is a genuine `transmute_lifetime`!
fn get_dangling<'a>() -> &'a u64 {
    let x = 0;
    transmute_lifetime(&x)
}

fn main() {
    let r = get_dangling();
    println!("{}", r);
}
