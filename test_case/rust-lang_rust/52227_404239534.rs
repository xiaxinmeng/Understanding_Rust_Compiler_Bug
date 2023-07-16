
     Running `/usr/bin/rustc --crate-name proc_macro2 vendor/proc-macro2/src/lib.rs --crate-type lib --emit=dep-info,link -C debug-assertions=off -C overflow-checks=on --cfg 'feature="default"' --cfg 'feature="proc-macro"' -C metadata=7ff6799d5f803168 -C extra-filename=-7ff6799d5f803168 --out-dir /builddir/build/BUILD/rustc-1.27.1-src/build/bootstrap/debug/deps -L dependency=/builddir/build/BUILD/rustc-1.27.1-src/build/bootstrap/debug/deps --extern unicode_xid=/builddir/build/BUILD/rustc-1.27.1-src/build/bootstrap/debug/deps/libunicode_xid-df8fd254b50536c5.rlib --cap-lints allow -Cdebuginfo=2`                                                                
error[E0463]: can't find crate for `proc_macro`
  --> vendor/proc-macro2/src/lib.rs:31:1
   |
31 | extern crate proc_macro;
   | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
