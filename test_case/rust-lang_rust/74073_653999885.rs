
2020-07-06T01:42:44.6886637Z thread 'rustc' panicked at 'attempted .def_id() on invalid res: NonMacroAttr(Builtin)', /checkout/src/libstd/macros.rs:16:9
2020-07-06T01:42:44.6894068Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-06T01:42:44.6898734Z 
2020-07-06T01:42:44.6908696Z error: internal compiler error: unexpected panic
2020-07-06T01:42:44.6913450Z 
2020-07-06T01:42:44.6921766Z note: the compiler unexpectedly panicked. this is a bug.
2020-07-06T01:42:44.6926813Z 
2020-07-06T01:42:44.6933768Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2020-07-06T01:42:44.6995882Z 
2020-07-06T01:42:44.6996824Z note: rustc 1.46.0-nightly (0c03aee8b 2020-07-05) running on x86_64-unknown-linux-gnu
2020-07-06T01:42:44.6997124Z 
2020-07-06T01:42:44.6998040Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2020-07-06T01:42:44.6998532Z 
2020-07-06T01:42:44.6998800Z note: some of the compiler flags provided by cargo are hidden
