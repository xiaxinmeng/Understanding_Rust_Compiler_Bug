rust
trait Alias = std::io::Write;

// The cfg result depends on whether the `Alias` refers to `io::Write`
#[cfg(::std::fs::File::write)]
