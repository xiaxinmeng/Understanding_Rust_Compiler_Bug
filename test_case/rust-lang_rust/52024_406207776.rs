plain
[00:36:51]    Compiling aho-corasick v0.6.6
[00:36:56]    Compiling tempfile v3.0.2
[00:37:16]    Compiling minifier v0.0.14
[00:37:19]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:19] error[E0532]: expected tuple struct/variant, found struct `clean::Existential`
[00:37:19]     --> librustdoc/html/render.rs:2105:13
[00:37:19]      |
[00:37:19] 2105 |             clean::Existential(..) => write!(fmt, "Existential Type ")?,
[00:37:19]      |             ^^^^^^^^^^^^^^^^^^ did you mean `clean::Existential { /* fields */ }`?
[00:37:19] help: possible better candidates are found in other modules, you can import them into scope
[00:37:19] 36   | use rustc::hir::ImplItemKind::Existential;
[00:37:19]      |
[00:37:19] 36   | use rustc::hir::ItemKind::Existential;
[00:37:19]      |
---
[00:37:23] 
[00:37:23] 
[00:37:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:37:23] Build completed unsuccessfully in 0:33:40
[00:37:23] Makefile:28: recipe for target 'all' failed
[00:37:23] make: *** [all] Error 1
342204 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu
251944 ./src/llvm/test
241184 ./src/llvm-emscripten
192644 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
---
154424 ./.git/modules/src
149120 ./src/llvm-emscripten/test
145348 ./obj/build/bootstrap/debug/incremental
130536 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130532 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f31lzoojnw-1d19v0j-2iqfvo5raelnm
118848 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
115720 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
108548 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
107616 ./src/llvm/test/CodeGen
---
travis_time:end:150a7c3c:start=1531991035199420569,finish=1531991035207393601,duration=7973032
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06b0e7a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0da042f3
travis_time:start:0da042f3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clan[   10.694515] init: failsafe main process (1093) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
