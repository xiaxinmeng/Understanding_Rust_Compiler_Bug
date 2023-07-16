
trait Foo {
    fn get_val(&self) -> int;
    fn do_stuff(&self @ SomeGarbage(a, b)) -> int { self.get_val() + 2 }
}
