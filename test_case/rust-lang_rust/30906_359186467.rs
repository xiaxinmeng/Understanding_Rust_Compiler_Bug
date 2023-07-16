rust
trait SomeTrait {}

fn foo<P, Q, F>(func: F)
where
    for<'a> F: Fake<'a, P, Q>,
    // just for an impl
    P: Default,
    Q: std::fmt::Debug,
{
    let p = P::default();
    let q = func(&p);
    println!("{:?}", q);
}

trait Fake<'a, P: 'a, Q>: Fn(&'a P) -> Q
where
    Self::Output: SomeTrait + 'a,
{
}

impl<'a, P: 'a, Q, F> Fake<'a, P, Q> for F
where
    F: Fn(&'a P) -> Q,
    F::Output: SomeTrait + 'a,
{
}

#[derive(Debug)]
struct Wrapper<'a>(&'a u8);

impl<'a> SomeTrait for Wrapper<'a> {}

fn main() {
    foo(Wrapper);
}
