plain
[00:41:09]    Compiling aho-corasick v0.6.4
[00:41:16]    Compiling tempdir v0.3.7
[00:41:55]    Compiling minifier v0.0.11
[00:41:58]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:41:59] error[E0532]: expected unit struct/variant or constant, found tuple variant `hir::Visibility::Crate`
[00:41:59]     --> librustdoc/clean/mod.rs:3033:13
[00:41:59]      |
[00:41:59] 3033 |             hir::Visibility::Crate => Visibility::Crate,
[00:41:59]      |             ^^^^^^^^^^^^^^^^^^^^^^ not a unit struct/variant or constant
[00:41:59] help: possible better candidates are found in other modules, you can import them into scope
[00:41:59] 14   | use clean::Visibility::Crate;
[00:41:59]      |
[00:41:59]      |
[00:41:59] 14   | use rustc::session::search_paths::PathKind::Crate;
[00:41:59] 14   | use syntax_pos::symbol::keywords::Crate;
[00:41:59]      |
[00:41:59] 
[00:42:16] error: aborting due to previous error
---
[00:42:16] 
[00:42:16] 
[00:42:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:42:16] Build completed unsuccessfully in 0:36:32
[00:42:16] Makefile:28: recipe for target 'all' failed
[00:42:16] make: *** [all] Error 1
18668 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
18316 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps
15152 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
15148 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
