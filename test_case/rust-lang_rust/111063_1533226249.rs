
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/gix-0.39.0/src/config/cache/incubate.rs:102:9
    |
85  |     buf: &mut Vec<u8>,
    |     --- move occurs because `buf` has type `&mut Vec<u8>`, which does not implement the `Copy` trait
...
99  |     std::io::copy(&mut file, buf)?;
    |                              --- value moved here
...
102 |         buf,
    |         ^^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
