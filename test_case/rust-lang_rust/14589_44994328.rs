 Rust
fn main() {
    send::<Box<Foo>>(box Output(0));               // 1  free function
    Test::<Box<Foo>>::foo(box Output(0));          // 2  static function
    Test::<Box<Foo>>.send(box Output(0));          // 3  method
}

fn send<T>(_: T) {}

struct Test<T>;
impl<T> Test<T> {
    fn foo(_: T) {}
    fn send(&self, _: T) {}
}

trait Foo {}
struct Output(int);
impl Foo for Output {}
