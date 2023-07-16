
struct ProxyObject<'q> (&'q B);

impl<'q> A for ProxyObject<'q> {
   fn f(&self) {let &ProxyObject(x) = self;  x.f();}
}

// &ProxyObject(b) as &A
