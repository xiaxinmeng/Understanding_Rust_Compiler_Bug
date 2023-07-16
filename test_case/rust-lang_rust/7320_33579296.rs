
trait Foo {
    fn foo(&self) { bar(self); }
}

fn bar(_b: &Foo) { }

fn main() {}
