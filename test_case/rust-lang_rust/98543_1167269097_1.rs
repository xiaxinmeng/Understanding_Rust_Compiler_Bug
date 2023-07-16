rust
trait Trait<'a>: 'a {
    type Type;
}

impl<'a, T> Trait<'a> for T
where
    T: 'a // if this bound get's implied we would probably get ub here again
{
    type Type = ();
}

fn f<'a, 'b>(s: &'b str, _: <&'b () as Trait<'a>>::Type) -> &'a str
where
    &'b (): Trait<'a>, // <- adding this bound is the change from #91068 
{
    s
}

fn main() {
    let x = String::from("Hello World!");
    let y = f(&x, ());
    drop(x);
    println!("{}", y);
}
