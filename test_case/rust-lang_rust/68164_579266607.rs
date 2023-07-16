rust
#[sanitizer(division_by_zero(never)]
#[sanitizer(signed_integer_overflow(never))]
fn stuff() { ... }
