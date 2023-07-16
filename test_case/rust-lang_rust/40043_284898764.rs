rust
trait Static = 'static;
fn foo(_: Box<Static>) { }
