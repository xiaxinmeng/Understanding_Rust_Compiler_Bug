rust
struct Foo;

fn takes_ref(_r: &Foo) {}

fn returns_result() -> Result<Foo,()> {}

fn main() -> Result<(),()> {
    takes_ref(returns_result()?);
    Ok(())
}
