 rust
fn say_something(&self)  -> Result {
    try!(self.term.reset());
    write!(term, "something");
}

fn do_something_important() {
    let _ = sayer.say_something();
}
