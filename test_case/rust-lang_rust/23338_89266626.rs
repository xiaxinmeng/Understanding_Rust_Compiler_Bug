 rust
struct D(&'static str, u32);

impl Drop for D {
    fn drop(&mut self) {
        println!("Dropping D({}, {})", self.0, self.1);
    }
}

impl D {
    fn incr(&self, name: &'static str) -> D {
        D(name, self.1 + 1)
    }
}

#[cfg(not(nested))]
fn foo(a1: D, b1: D) -> (&'static str, D) {
    let _b2 = b1.incr("b");

        let _a2 = a1.incr("a");
        let b3 = b1.incr("temp1").incr("b");
        println!("made b2 a2 and b3");
        ("foo_direct", b3.incr("temp2").incr("b"))

}

#[cfg(nested)]
fn foo(a1: D, b1: D) -> (&'static str, D) {
    let _b2 = b1.incr("b");
    {
        let _a2 = a1.incr("a");
        let b3 = b1.incr("temp1").incr("b");
        println!("made b2 a2 and b3");
        ("foo_nested", b3.incr("temp2").incr("b"))
    }
}

fn main() {
    let (name, result) = foo(D("param_a", 1), D("param_b", 1));
    println!("called {}, got result D({}, {})",
             name, result.0, result.1);
}
