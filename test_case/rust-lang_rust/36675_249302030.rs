 rust
extern crate bob;

unsafe fn not_safe() -> u8 { bob::s::unsafe_function(0, "foo") }

fn guaranteed_to_always_be_safe() -> u8 { unsafe { bob::s::unsafe_function(1, "foo") } }
