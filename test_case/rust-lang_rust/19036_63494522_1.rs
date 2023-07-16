 rust
// libcore/str.rs
struct str([u8]);

impl str { fn contains__(&self, needle: &str) -> bool { /* .. */ } }

// and this will just work?
let foo: &'static str = "Hello!";
assert!(foo.contains__("el"));
