rust
auto trait Trait1<'a> {}

fn f<'a>(x: &dyn Trait1<'a>)
{}

fn main() {
    f(&1);
}
