rust
#![feature(generic_associated_types)]

trait Trait<'a> {
    type Type
    where
        Self: 'a;
}

impl<'a, T> Trait<'a> for T {
    type Type = ()
    where
        Self: 'a; // once this bound gets implied, we have UB again
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
