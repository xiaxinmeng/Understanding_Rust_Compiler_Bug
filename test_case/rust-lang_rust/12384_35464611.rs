 rust
fn foo(x: int) {
     println!(x); // note: format string literal missing, using default
     bar(x + 1);
}
fn bar(x: int) {
    println!(x); // note: format string literal missing, using default
    baz(x + 2);
}
fn baz(x: int) {
    println!(x); // note: format string literal missing, using default
    // ...
}
fn main() { foo(0) }
