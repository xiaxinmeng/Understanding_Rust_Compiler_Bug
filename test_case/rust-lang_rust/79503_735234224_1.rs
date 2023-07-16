
→ rustup show active-toolchain
stable-x86_64-unknown-linux-gnu (default)
→ cargo test -p clap_derive --test nested --features "wrap_help yaml regex"
error[E0425]: cannot find value `arg_matches` in this scope
Error:   --> clap_derive/tests/nested.rs:32:26
   |
32 |     expand_ty!(my_field: Option<String>);
   |                          ^^^^^^ not found in this scope
