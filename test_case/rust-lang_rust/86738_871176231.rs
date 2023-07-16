rust
struct Foo;

fn foo() -> Result<(), Foo> {
    let _k = Foo;
    let inner = || {
        if false {
            // Uncommenting this fixes compilation
            // return Err(Foo);
        }

        // _x's type is unknown!
        let _x = Err(Foo)?;
        Ok(())
    };
    inner()?;
    Ok(())
}

struct Bar;

// This compiles just fine
impl From<Foo> for Bar {
    fn from(_: Foo) -> Bar {
        Bar
    }
}

// This causes compilation to fail
impl From<Bar> for Foo {
    fn from(_: Bar) -> Foo {
        Foo
    }
}

fn main() {
    println!("Hello, world!");
}
