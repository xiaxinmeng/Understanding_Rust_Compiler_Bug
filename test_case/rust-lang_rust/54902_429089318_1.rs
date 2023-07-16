rust
trait WithType<T> {}
trait WithRegion<'a> { }

struct Foo<T> { 
    t: T
}

impl<T> Foo<T>
where T: WithType<&u32> { }

fn main() {}
