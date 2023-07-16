 rust
trait SomeTrait {
    fn dummy(&self) { }
}

impl SomeTrait for i32 { }

fn arr<'a>(x: &mut [Box<SomeTrait+'a>]) {
}

fn main() {
    arr(&mut [Box::new(1)])
}
