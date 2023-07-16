 rust
// core.rc

...

// FIXME(#3114): Macro import/export.
fn macros() { include!("integer-templates.rs"); }

/* Primitive types */

// pub mod i8 { int_template!(i8, 8) } // what it'll look like w/o temporary workaround below

pub mod i8  {
    int_template!(i8, 8) // accepts type and number of bits
    pub use i8::inst::*;
}
...
