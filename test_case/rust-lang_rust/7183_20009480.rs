 rust
#[allow(default_methods)]
trait Speak { fn say(&self, s:&str); fn hi(&self) { hello(self); } }
fn hello<S:Speak>(s:&S) { s.say("hello"); }
