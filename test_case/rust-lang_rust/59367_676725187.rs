
$ cargo doc
 Documenting inner v0.1.0 (/home/joshua/test-rustdoc/inner)
    Checking inner v0.1.0 (/home/joshua/test-rustdoc/inner)
warning: unresolved link to `broken`
 --> /home/joshua/test-rustdoc/inner/src/lib.rs:1:11
  |
1 | /// oops [broken] link
  |           ^^^^^^ unresolved link
  |
  = note: `#[warn(intra_doc_link_resolution_failure)]` on by default
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: 1 warning emitted

 Documenting outer v0.1.0 (/home/joshua/test-rustdoc/outer)
    Finished dev [unoptimized + debuginfo] target(s) in 0.97s
$ touch ../inner/src/lib.rs 
$ cargo doc --no-deps
    Checking inner v0.1.0 (/home/joshua/test-rustdoc/inner)
 Documenting outer v0.1.0 (/home/joshua/test-rustdoc/outer)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
