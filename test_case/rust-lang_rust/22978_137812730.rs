
struct A { }
struct B { }
impl Foo for A { }
impl Foo for B { }
impl !Send for (A, B) { }
