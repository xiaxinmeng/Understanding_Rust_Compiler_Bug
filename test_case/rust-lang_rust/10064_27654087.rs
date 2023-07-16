
fn foo(f: extern "C" fn(*c_char, ...)) { ... }
fn main() {
    foo(sprintf);
}
