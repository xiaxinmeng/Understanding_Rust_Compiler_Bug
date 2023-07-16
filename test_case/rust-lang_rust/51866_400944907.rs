plain
[00:36:26]    Compiling aho-corasick v0.6.4
[00:36:33]    Compiling tempfile v3.0.2
[00:37:08]    Compiling minifier v0.0.14
[00:37:12]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:13] error[E0532]: expected unit struct/variant or constant, found tuple variant `hir::Visibility::Public`
[00:37:13]     --> librustdoc/clean/mod.rs:3229:13
[00:37:13]      |
[00:37:13] 3229 |             hir::Visibility::Public => Visibility::Public,
[00:37:13]      |             ^^^^^^^^^^^^^^^^^^^^^^^ not a unit struct/variant or constant
[00:37:13] help: possible better candidates are found in other modules, you can import them into scope
[00:37:13] 14   | use clean::Visibility::Public;
[00:37:13]      |
[00:37:13]      |
[00:37:13] 14   | use minifier::js::Keyword::Public;
[00:37:13]      |
[00:37:13] 14   | use rustc::middle::privacy::AccessLevel::Public;
[00:37:13] 14   | use rustc::ty::Visibility::Public;
[00:37:13]      |
[00:37:13] and 1 other candidates
[00:37:13] 
[00:37:13] 
[00:37:17] error[E0308]: mismatched types
[00:37:17]    --> librustdoc/clean/mod.rs:289:36
[00:37:17]     |
[00:37:17] 289 |                     if item.vis == hir::Visibility::Public => {
[00:37:17]     |                                    ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:17]     = note: expected type `rustc::hir::Visibility`
[00:37:17]     = note: expected type `rustc::hir::Visibility`
[00:37:17]                found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:17] error[E0308]: mismatched types
[00:37:17]    --> librustdoc/clean/mod.rs:331:36
[00:37:17]     |
[00:37:17]     |
[00:37:17] 331 |                     if item.vis == hir::Visibility::Public => {
[00:37:17]     |                                    ^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:17]     = note: expected type `rustc::hir::Visibility`
[00:37:17]     = note: expected type `rustc::hir::Visibility`
[00:37:17]                found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:20] error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
[00:37:20]     --> librustdoc/clean/mod.rs:3231:13
[00:37:20]      |
[00:37:20]      |
[00:37:20] 3231 |             hir::Visibility::Crate(_) => Visibility::Crate,
[00:37:20] 
[00:37:22] error[E0308]: mismatched types
[00:37:22]     --> librustdoc/clean/mod.rs:3935:34
[00:37:22]      |
[00:37:22]      |
[00:37:22] 3935 |         let denied = self.vis != hir::Public || self.attrs.iter().any(|a| {
[00:37:22]      |                                  ^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:22]      = note: expected type `rustc::hir::Visibility`
[00:37:22]      = note: expected type `rustc::hir::Visibility`
[00:37:22]                 found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:29] error[E0308]: mismatched types
[00:37:29]   --> librustdoc/visit_ast.rs:97:47
[00:37:29]    |
[00:37:29] 97 |                                               hir::Public,
[00:37:29] 97 |                                               hir::Public,
[00:37:29]    |                                               ^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:29]    = note: expected type `rustc::hir::Visibility`
[00:37:29]    = note: expected type `rustc::hir::Visibility`
[00:37:29]               found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:29] error[E0308]: mismatched types
[00:37:29]    --> librustdoc/visit_ast.rs:207:43
[00:37:29]     |
[00:37:29]     |
[00:37:29] 207 |         self.inside_public_path &= vis == hir::Public;
[00:37:29]     |                                           ^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:29]     = note: expected type `rustc::hir::Visibility`
[00:37:29]     = note: expected type `rustc::hir::Visibility`
[00:37:29]                found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:30] error[E0308]: mismatched types
[00:37:30]    --> librustdoc/visit_ast.rs:379:24
[00:37:30]     |
[00:37:30]     |
[00:37:30] 379 |         if item.vis == hir::Public {
[00:37:30]     |                        ^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:30]     = note: expected type `rustc::hir::Visibility`
[00:37:30]     = note: expected type `rustc::hir::Visibility`
[00:37:30]                found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:31] error[E0308]: mismatched types
[00:37:31]    --> librustdoc/visit_ast.rs:390:68
[00:37:31]     |
[00:37:31]     |
[00:37:31] 390 |                         items: fm.items.iter().filter(|i| i.vis == hir::Public).cloned().collect(),
[00:37:31]     |                                                                    ^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:31]     = note: expected type `rustc::hir::Visibility`
[00:37:31]     = note: expected type `rustc::hir::Visibility`
[00:37:31]                found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:31] error[E0308]: mismatched types
[00:37:31]    --> librustdoc/visit_ast.rs:397:47
[00:37:31]     |
[00:37:31]     |
[00:37:31] 397 |             _ if self.inlining && item.vis != hir::Public => {}
[00:37:31]     |                                               ^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:31]     = note: expected type `rustc::hir::Visibility`
[00:37:31]     = note: expected type `rustc::hir::Visibility`
[00:37:31]                found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:32] error[E0308]: mismatched types
[00:37:32]    --> librustdoc/visit_ast.rs:417:32
[00:37:32]     |
[00:37:32]     |
[00:37:32] 417 |                 if item.vis == hir::Public && self.inside_public_path {
[00:37:32]     |                                ^^^^^^^^^^^ expected enum `rustc::hir::Visibility`, found fn item
[00:37:32]     = note: expected type `rustc::hir::Visibility`
[00:37:32]     = note: expected type `rustc::hir::Visibility`
[00:37:32]                found type `fn(syntax_pos::Span) -> rustc::hir::Visibility {rustc::hir::Visibility::Public}`
[00:37:32] error: aborting due to 11 previous errors
[00:37:32] 
[00:37:32] Some errors occurred: E0023, E0308, E0532.
[00:37:32] For more information about an error, try `rustc --explain E0023`.
---
[00:37:32] 
[00:37:32] 
[00:37:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:37:32] Build completed unsuccessfully in 0:32:41
[00:37:32] Makefile:28: recipe for target 'all' failed
[00:37:32] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ace9bf3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00278a2a:start=1530172171413818708,finish=1530172171421374024,duration=7555316
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01e0dc46
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3172769a
$ dmesg | grep -i kill
