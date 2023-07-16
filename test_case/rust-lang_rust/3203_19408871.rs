
trait Hello { fn hello(self); }
impl Hello for &fn() {
    fn hello(self) { self(); }
}
fn hello<T: Hello>(x: T) {
    x.hello();
}
fn main() {
    do hello { error!("Hello world"); }
}
