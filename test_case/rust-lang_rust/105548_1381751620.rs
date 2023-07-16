rust
trait Trait {
    type Type;
}

impl<T> Trait for T {
    type Type = ();
}

trait Extend<'a, 'b> {
    fn extend(s: &'a str) -> &'b str;
}

impl<'a, 'b> Extend<'a, 'b> for <&'b &'a () as Trait>::Type
where
    for<'what, 'ever> &'what &'ever (): Trait,
{
    // instead of getting the implied bound from the self type
    // we get it through an explicit bound on the impl method instead.
    //
    // This explicit bound is accepted because it's implied by the
    // impl header.
    fn extend(s: &'a str) -> &'b str
    where
        'a: 'b
    {
        s
    }
}

fn main() {
    let y = <() as Extend<'_, '_>>::extend(&String::from("Hello World"));
    println!("{}", y);
}
