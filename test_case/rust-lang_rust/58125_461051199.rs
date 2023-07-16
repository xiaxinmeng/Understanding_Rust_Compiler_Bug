plain
travis_time:end:05153590:start=1549459209293998767,finish=1549459212440707061,duration=3146708294
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:55] 
[01:08:55] running 119 tests
[01:09:19] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:09:23] i......iii.i.....ii
[01:09:23] 
[01:09:23]  finished in 28.611
[01:09:23] travis_fold:end:test_debuginfo

---
[01:32:13]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:32:15] error[E0433]: failed to resolve: use of undeclared type or module `util`
[01:32:15]     --> src/libsyntax/mut_visit.rs:1261:9
[01:32:15]      |
[01:32:15] 1261 |     use util::parser_testing::{string_to_crate, matches_codepattern};
[01:32:15] 
[01:32:15] error[E0432]: unresolved import `ast`
[01:32:15]     --> src/libsyntax/mut_visit.rs:1260:15
[01:32:15]      |
[01:32:15]      |
[01:32:15] 1260 |     use ast::{self, Ident};
[01:32:15]      |               ^^^^ no `ast` external crate
[01:32:15] error[E0432]: unresolved import `print`
[01:32:15]     --> src/libsyntax/mut_visit.rs:1262:9
[01:32:15]      |
[01:32:15] 1262 |     use print::pprust;
[01:32:15] 1262 |     use print::pprust;
[01:32:15]      |         ^^^^^ did you mean `crate::print`?
[01:32:15]      |
[01:32:15]      = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[01:32:15] error[E0432]: unresolved import `mut_visit`
[01:32:15]     --> src/libsyntax/mut_visit.rs:1263:9
[01:32:15]      |
[01:32:15] 1263 |     use mut_visit;
[01:32:15] 1263 |     use mut_visit;
[01:32:15]      |         ^^^^^^^^^ no `mut_visit` external crate
[01:32:15] 
[01:32:15] error[E0432]: unresolved import `with_globals`
[01:32:15]     --> src/libsyntax/mut_visit.rs:1264:9
[01:32:15] 1264 |     use with_globals;
[01:32:15] 1264 |     use with_globals;
[01:32:15]      |         ^^^^^^^^^^^^ no `with_globals` external crate
[01:32:17] error[E0425]: cannot find function `string_to_crate` in this scope
[01:32:17]     --> src/libsyntax/mut_visit.rs:1304:29
[01:32:17]      |
[01:32:17] 1304 |             let mut krate = string_to_crate(
---
[01:32:17] 1308 |                 matches_codepattern,
[01:32:17]      |                 ^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:32:17] help: possible candidate is found in another module, you can import it into scope
[01:32:17]      |
[01:32:17] 1259 |     use crate::util::parser_testing::matches_codepattern;
[01:32:17] 
[01:32:17] error[E0425]: cannot find function `string_to_crate` in this scope
[01:32:17]     --> src/libsyntax/mut_visit.rs:1319:29
[01:32:17]      |
---
[01:32:17] 1324 |                 matches_codepattern,
[01:32:17]      |                 ^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:32:17] help: possible candidate is found in another module, you can import it into scope
[01:32:17]      |
[01:32:17] 1259 |     use crate::util::parser_testing::matches_codepattern;
[01:32:17] 
[01:32:17] error: unused import: `Ident`
[01:32:17]     --> src/libsyntax/mut_visit.rs:1260:21
[01:32:17]      |
[01:32:17]      |
[01:32:17] 1260 |     use ast::{self, Ident};
[01:32:17]      |
[01:32:17]      = note: `-D unused-imports` implied by `-D warnings`
[01:32:17] 
[01:32:17] 
[01:32:17] error: unused imports: `matches_codepattern`, `string_to_crate`
[01:32:17]     --> src/libsyntax/mut_visit.rs:1261:32
[01:32:17]      |
[01:32:17] 1261 |     use util::parser_testing::{string_to_crate, matches_codepattern};
[01:32:17] 
[01:32:24] error: aborting due to 11 previous errors
[01:32:24] 
[01:32:24] Some errors occurred: E0425, E0432, E0433.
[01:32:24] Some errors occurred: E0425, E0432, E0433.
[01:32:24] For more information about an error, try `rustc --explain E0425`.
[01:32:24] error: Could not compile `syntax`.
[01:32:24] 
[01:32:24] To learn more, run the command again with --verbose.
[01:32:24] 
[01:32:24] 
[01:32:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:32:24] 
[01:32:24] 
[01:32:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:24] Build completed unsuccessfully in 0:34:56
[01:32:24] Build completed unsuccessfully in 0:34:56
[01:32:24] Makefile:48: recipe for target 'check' failed
[01:32:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0307bb75
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 14:52:46 UTC 2019
