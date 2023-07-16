rust
struct Foo { }
impl Foo { const Constant: usize = 22; }
enum Bar<Foo> { X = Foo::Constant }
