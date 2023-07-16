

and somehow it seems to "work" in the sense that `cargo test -- --ignored` does not pick it up, *and* `cargo doc` uses Rust syntax highlighting for that code block!

Some things I tried didn't work: obviously this is a horrible hack and probably happens to work for now because of some bug in rustdoc or whatever, but just sharing it here in case it's of interest to anyone :-)