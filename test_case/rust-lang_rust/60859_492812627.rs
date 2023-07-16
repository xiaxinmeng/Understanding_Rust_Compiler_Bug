plain
[01:24:15]     |
[01:24:15] 255 |                         .map(LocalInternedString::get)
[01:24:15]     |                          ^^^
[01:24:15]     |                          |
[01:24:15]     |                          expected signature of `fn(&syntax_pos::Symbol) -> _`
[01:24:15]     |                          found signature of `for<'r> fn(&'r syntax::symbol::LocalInternedString) -> _`
[01:24:15] 
[01:24:15] error[E0599]: no method named `collect` found for type `std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}>` in the current scope
[01:24:15]     |
[01:24:15] 256 |                         .collect::<Vec<_>>();
[01:24:15]     |                          ^^^^^^^
[01:24:15]     |
[01:24:15]     |
[01:24:15]     = note: the method `collect` exists but the following trait bounds were not satisfied:
[01:24:15]             `&mut std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}> : std::iter::Iterator`
[01:24:15]             `std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}> : std::iter::Iterator`
[01:24:15] error[E0308]: mismatched types
[01:24:15]     --> src/tools/clippy/clippy_lints/src/utils/mod.rs:1124:28
[01:24:15]      |
[01:24:15]      |
[01:24:15] 1124 |     cx.match_def_path(did, &syms)
[01:24:15]      |
[01:24:15]      |
[01:24:15]      = note: expected type `&[syntax_pos::Symbol]`
[01:24:15]                 found type `&std::vec::Vec<&str>`
[01:24:17] error[E0631]: type mismatch in function arguments
[01:24:17]    --> src/tools/clippy/clippy_lints/src/consts.rs:255:26
[01:24:17]     |
[01:24:17] 255 |                         .map(LocalInternedString::get)
[01:24:17] 255 |                         .map(LocalInternedString::get)
[01:24:17]     |                          ^^^
[01:24:17]     |                          |
[01:24:17]     |                          expected signature of `fn(&syntax_pos::Symbol) -> _`
[01:24:17]     |                          found signature of `for<'r> fn(&'r syntax::symbol::LocalInternedString) -> _`
[01:24:17] 
[01:24:17] error[E0599]: no method named `collect` found for type `std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}>` in the current scope
[01:24:17]     |
[01:24:17] 256 |                         .collect::<Vec<_>>();
[01:24:17]     |                          ^^^^^^^
[01:24:17]     |
[01:24:17]     |
[01:24:17]     = note: the method `collect` exists but the following trait bounds were not satisfied:
[01:24:17]             `&mut std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}> : std::iter::Iterator`
[01:24:17]             `std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}> : std::iter::Iterator`
[01:24:17] error[E0308]: mismatched types
[01:24:17]     --> src/tools/clippy/clippy_lints/src/utils/mod.rs:1124:28
[01:24:17]      |
[01:24:17]      |
[01:24:17] 1124 |     cx.match_def_path(did, &syms)
[01:24:17]      |
[01:24:17]      |
[01:24:17]      = note: expected type `&[syntax_pos::Symbol]`
[01:24:17]                 found type `&std::vec::Vec<&str>`
[01:24:18] error[E0308]: mismatched types
[01:24:18]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:24
[01:24:18]      |
[01:24:18]      |
[01:24:18] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:24:18]      |
[01:24:18]      = note: expected type `syntax_pos::Symbol`
[01:24:18]                 found type `&'static str`
[01:24:18] 
[01:24:18] 
[01:24:18] error[E0308]: mismatched types
[01:24:18]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:46
[01:24:18]      |
[01:24:18] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:24:18]      |
[01:24:18]      = note: expected type `syntax_pos::Symbol`
[01:24:18]                 found type `&'static str`
[01:24:18] 
[01:24:18] 
[01:24:19] error[E0308]: mismatched types
[01:24:19]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:82
[01:24:19]      |
[01:24:19] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:24:19]      |
[01:24:19]      = note: expected type `syntax_pos::Symbol`
[01:24:19]                 found type `&'static str`
[01:24:19] 
---
[01:24:19] warning: build failed, waiting for other jobs to finish...
[01:24:20] error[E0308]: mismatched types
[01:24:20]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:24
[01:24:20]      |
[01:24:20] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:24:20]      |
[01:24:20]      = note: expected type `syntax_pos::Symbol`
[01:24:20]                 found type `&'static str`
[01:24:20] 
[01:24:20] 
[01:24:20] error[E0308]: mismatched types
[01:24:20]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:46
[01:24:20]      |
[01:24:20] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:24:20]      |
[01:24:20]      = note: expected type `syntax_pos::Symbol`
[01:24:20]                 found type `&'static str`
[01:24:20] 
[01:24:20] 
[01:24:20] error[E0308]: mismatched types
[01:24:20]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:82
[01:24:20]      |
[01:24:20] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:24:20]      |
[01:24:20]      = note: expected type `syntax_pos::Symbol`
[01:24:20]                 found type `&'static str`
[01:24:20] 
---
[01:27:50]     |
[01:27:50] 255 |                         .map(LocalInternedString::get)
[01:27:50]     |                          ^^^
[01:27:50]     |                          |
[01:27:50]     |                          expected signature of `fn(&syntax_pos::Symbol) -> _`
[01:27:50]     |                          found signature of `for<'r> fn(&'r syntax::symbol::LocalInternedString) -> _`
[01:27:50] 
[01:27:50] error[E0599]: no method named `collect` found for type `std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}>` in the current scope
[01:27:50]     |
[01:27:50] 256 |                         .collect::<Vec<_>>();
[01:27:50]     |                          ^^^^^^^
[01:27:50]     |
[01:27:50]     |
[01:27:50]     = note: the method `collect` exists but the following trait bounds were not satisfied:
[01:27:50]             `&mut std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}> : std::iter::Iterator`
[01:27:50]             `std::iter::Map<std::slice::Iter<'_, syntax_pos::Symbol>, for<'r> fn(&'r syntax::symbol::LocalInternedString) -> &'r str {syntax::symbol::LocalInternedString::get}> : std::iter::Iterator`
[01:27:50]    Compiling im-rc v12.3.0
[01:27:50] error[E0308]: mismatched types
[01:27:50]     --> src/tools/clippy/clippy_lints/src/utils/mod.rs:1124:28
[01:27:50]      |
[01:27:50]      |
[01:27:50] 1124 |     cx.match_def_path(did, &syms)
[01:27:50]      |
[01:27:50]      |
[01:27:50]      = note: expected type `&[syntax_pos::Symbol]`
[01:27:50]                 found type `&std::vec::Vec<&str>`
[01:27:52]    Compiling rls-analysis v0.17.0
[01:27:55] error[E0308]: mismatched types
[01:27:55]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:24
[01:27:55]      |
[01:27:55]      |
[01:27:55] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:27:55]      |
[01:27:55]      = note: expected type `syntax_pos::Symbol`
[01:27:55]                 found type `&'static str`
[01:27:55] 
[01:27:55] 
[01:27:55] error[E0308]: mismatched types
[01:27:55]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:46
[01:27:55]      |
[01:27:55] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:27:55]      |
[01:27:55]      = note: expected type `syntax_pos::Symbol`
[01:27:55]                 found type `&'static str`
[01:27:55] 
[01:27:55] 
[01:27:56] error[E0308]: mismatched types
[01:27:56]     --> src/tools/clippy/clippy_lints/src/types.rs:1096:82
[01:27:56]      |
[01:27:56] 1096 |         if names[0] == "libc" || names[0] == "core" && *names.last().unwrap() == "c_void" {
[01:27:56]      |
[01:27:56]      = note: expected type `syntax_pos::Symbol`
[01:27:56]                 found type `&'static str`
[01:27:56] 
---
travis_time:end:0bf6c008:start=1557952232329558052,finish=1557952232350635646,duration=21077594
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06fbc065
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12a2f3f0
travis_time:start:12a2f3f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f102455
$ dmesg | grep -i kill
