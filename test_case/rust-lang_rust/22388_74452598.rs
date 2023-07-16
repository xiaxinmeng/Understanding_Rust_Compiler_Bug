 rust
// this is a valid crate as per the current orphan check rules
extern crate your;

use your::Foo;

struct Mine;

impl Foo for *const Mine { .. }
