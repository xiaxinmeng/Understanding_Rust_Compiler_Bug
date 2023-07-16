rust
trait Trait<'a> { }
impl<'a, T> Trait<'a> for T { }

fn foo<'x, T>(t: T)
where T: Trait<'x>
{ 
}

fn main() { 
  foo(22);
}
