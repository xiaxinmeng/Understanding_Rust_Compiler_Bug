
pub struct Foo { .. } // The non-exhaustive struct
const Foo: Foo = Foo {}; // Optional convenience: a private constructor to avoid writing `{}`s in the current crate if it needs to be written often
