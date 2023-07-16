rs
struct A(u8);
struct B(u8);
struct C(u8);
struct D(u8);


impl PartialEq<B> for A {
    fn eq(&self, other: &B) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<A> for B {
    fn eq(&self, other: &A) -> bool {
        self.0 == other.0
    }
}


impl PartialEq<C> for B {
    fn eq(&self, other: &C) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<B> for C {
    fn eq(&self, other: &B) -> bool {
        self.0 == other.0
    }
}


impl PartialEq<D> for C {
    fn eq(&self, other: &D) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<C> for D {
    fn eq(&self, other: &C) -> bool {
        self.0 == other.0
    }
}


impl PartialEq<A> for D {
    fn eq(&self, other: &A) -> bool {
        true
    }
}
impl PartialEq<D> for A {
    fn eq(&self, other: &D) -> bool {
        true
    }
}
