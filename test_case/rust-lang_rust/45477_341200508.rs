rust
use crate::foo::bar;

fn baz() {
    bar(); // equivalent to next line
    ::crate::foo::bar();
}
