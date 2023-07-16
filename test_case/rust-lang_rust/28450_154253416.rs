 rust
type Foo = Arc<Mutex<io::Read + Send + 'static>>;
pub fn foo(x: Foo) {}
