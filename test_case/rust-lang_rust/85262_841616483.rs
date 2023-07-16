
     Running `rustc --crate-name vte --edition=2018 /home/wucke13/.cargo/registry/src/github.com-1ecc6299db9ec823/vte-0.9.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="arrayvec"' --cfg 'feature="default"' --cfg 'feature="no_std"' -C metadata=cada7e2728fb1dfa -C extra-filename=-cada7e2728fb1dfa --out-dir /tmp/bad_apple/target/debug/deps -L dependency=/tmp/bad_apple/target/debug/deps --extern arrayvec=/tmp/bad_apple/target/debug/deps/libarrayvec-01054b7e1cbbf029.rmeta --extern utf8parse=/tmp/bad_apple/target/debug/deps/libutf8parse-bf36fb811c7b133e.rmeta --extern vte_generate_state_changes=/tmp/bad_apple/target/debug/deps/libvte_generate_state_changes-c32257cda334b594.so --cap-lints allow`
error: /nix/store/0c7c96gikmzv87i7lv3vq5s1cmfjd6zf-glibc-2.31-74/lib/libc.so.6: version `GLIBC_2.32' not found (required by /tmp/bad_apple/target/debug/deps/libvte_generate_state_changes-c32257cda334b594.so)
 --> /home/wucke13/.cargo/registry/src/github.com-1ecc6299db9ec823/vte-0.9.0/src/table.rs:5:5
  |
5 | use vte_generate_state_changes::generate_state_changes;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: could not compile `vte`

Caused by:
  process didn't exit successfully: `rustc --crate-name vte --edition=2018 /home/wucke13/.cargo/registry/src/github.com-1ecc6299db9ec823/vte-0.9.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="arrayvec"' --cfg 'feature="default"' --cfg 'feature="no_std"' -C metadata=cada7e2728fb1dfa -C extra-filename=-cada7e2728fb1dfa --out-dir /tmp/bad_apple/target/debug/deps -L dependency=/tmp/bad_apple/target/debug/deps --extern arrayvec=/tmp/bad_apple/target/debug/deps/libarrayvec-01054b7e1cbbf029.rmeta --extern utf8parse=/tmp/bad_apple/target/debug/deps/libutf8parse-bf36fb811c7b133e.rmeta --extern vte_generate_state_changes=/tmp/bad_apple/target/debug/deps/libvte_generate_state_changes-c32257cda334b594.so --cap-lints allow` (exit status: 1)
warning: build failed, waiting for other jobs to finish...
error: `proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`
  --> /home/wucke13/.cargo/registry/src/github.com-1ecc6299db9ec823/time-macros-impl-0.1.1/src/lib.rs:84:13
   |
84 |               pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
91 | / impl_macros! {
92 | |     time: Time,
93 | |     offset: Offset,
94 | |     date: Date,
95 | | }
   | |_- in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: /nix/store/0c7c96gikmzv87i7lv3vq5s1cmfjd6zf-glibc-2.31-74/lib/libc.so.6: version `GLIBC_2.32' not found (required by /tmp/bad_apple/target/debug/deps/libproc_macro_hack-cd384c8b6ea8a8ba.so)
  --> /home/wucke13/.cargo/registry/src/github.com-1ecc6299db9ec823/time-macros-impl-0.1.1/src/lib.rs:75:5
   |
75 | use proc_macro_hack::proc_macro_hack;
   |     ^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors

error: build failed
