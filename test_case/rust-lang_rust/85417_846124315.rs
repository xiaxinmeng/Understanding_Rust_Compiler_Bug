rust
trait Something { type Item; }
fn foo(x: &dyn Something<Item = &u32>) { }   // works fine
fn foo(x: &impl Something<Item = &u32>) { }  // error
