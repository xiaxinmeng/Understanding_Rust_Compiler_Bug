 rust
#[deriving(ToStr)]
struct Foo<A>(A);
#[doc = "Automatically derived."]
pub impl <A: ::std::to_str::ToStr> ::std::to_str::ToStr for Foo<A> {
    pub fn to_str(&self) -> ~str {
        match *self { Foo(ref __self_0_0) => ::std::sys::log_str(&*self) }
    }
}
