 rust
// in src/libstd/io/mod.rs
#[doc(hidden)]
pub mod __print {
    pub fn println(args: &fmt::Arguments) { ... }
    pub fn print(args: &fmt::Arguments) { ... }
}
