
// in rust
#[link_name="bar"]
pub extern fn stuff(a: NotSoGood) { ... }

// somewhere else, still in rust
extern { fn bar(a: NotSoGood); }
