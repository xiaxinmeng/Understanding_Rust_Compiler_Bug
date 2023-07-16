plain
   Compiling addr2line v0.14.0
error: implementation has missing stability attribute
    --> library/std/src/io/mod.rs:2472:1
     |
2472 | / impl Iterator for Bytes<fs::File> {
2473 | |     fn size_hint(&self) -> (usize, Option<usize>) {
2474 | |         match self.inner.metadata() {
2475 | |             Ok(metadata) => {
2481 | |     }
2482 | | }
     | |_^

