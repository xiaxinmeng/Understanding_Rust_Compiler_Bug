 rust
#[no_std];

pub fn foo() -> int {
    static foo_constant: int = 1;
    return foo_constant;
}

struct A;

trait B {
    fn bar(&self) -> int;
}

impl A {
    fn foo(&self) -> int {
        static foo_constant: int = 1;
        return foo_constant + foo();
    }
}

impl B for A {
    fn bar(&self) -> int {
        static bar_constant: int = 1;
        return bar_constant + self.foo();
    }
}

struct C<T>;

impl<T> C<T> {
    fn baz(&self) -> int {
        static baz_constant: int = 1;
        return baz_constant;
    }
}

#[start]
fn main(_: int, _: **u8, _: *u8) -> int {
    let a = A;
    a.bar();
    let c = C::<()>;
    c.baz();
    3
}
