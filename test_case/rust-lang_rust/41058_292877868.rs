rust
trait VariadicTrait<Args> {
    fn variadic_fn(&self, args: Args);
}

impl VariadicTrait<(&'static str,)> for () {
    fn variadic_fn(&self, (s,): (&'static str,)) {
        println!("{}", s)
    }
}

fn main() {
    ().variadic_fn(("hi",))
}
