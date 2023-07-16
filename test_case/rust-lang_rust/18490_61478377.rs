 rust
trait Command { fn name() -> &'static str; }
struct Init;
impl Command for Init { fn name() -> &'static str { "init" } }
fn main() {  let cmd = box Init as Box<Command>; }
