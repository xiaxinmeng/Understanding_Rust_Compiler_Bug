 rust
use somemod::bar;

fn qux() { bar() }

fn main() {
    use othermod::*;

    qux();
    bar(); // supposed to be somemod's bar
    something_in_othermod();    
}
