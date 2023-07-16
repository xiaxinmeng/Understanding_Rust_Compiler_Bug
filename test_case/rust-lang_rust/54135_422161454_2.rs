 rust
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: fn() = reset;

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [fn(); 14] = [..];
