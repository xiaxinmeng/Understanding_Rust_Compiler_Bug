rust
struct S(i32);
struct T<'a>(&'a S);

// Uncomment this impl, and the program fails to compile.
// impl<'a> Drop for T<'a> { fn drop(&mut self) { } }

fn foo() -> S { S(42) }

impl S {
    fn bar(&self) -> T { T(self) }
}

impl<'a> T<'a> {
    fn get(&self) -> i32 { (self.0).0 }
}

fn main() {
    let _x = foo().bar().get();
}
