plain
travis_time:end:0674e524:start=1543871717704435269,finish=1543871778601430558,duration=60896995289
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:44] 
[00:57:44] running 120 tests
[00:57:48] i..ii...iii...iiii....i...i..........i..iii.............i......i....ii...i..i.ii..............i....i 100/120
[00:57:48] i.ii.i.....iiii.....
[00:57:48] 
[00:57:48]  finished in 3.641
[00:57:48] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:04] 
[00:58:04] running 118 tests
[00:58:28] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:58:32] ......iii.i.....ii
[00:58:32] 
[00:58:32]  finished in 28.548
[00:58:32] travis_fold:end:test_debuginfo

---
[01:24:21]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:24:25] error[E0308]: mismatched types
[01:24:25]    --> src/librustdoc/clean/cfg.rs:595:35
[01:24:25]     |
[01:24:25] 595 |             assert_eq!(Cfg::parse(&mi), Ok(word_cfg("all")));
[01:24:25]     |                                   ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:25]     |
[01:24:25]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:25] 
[01:24:26] error[E0308]: mismatched types
[01:24:26]    --> src/librustdoc/clean/cfg.rs:605:35
[01:24:26]     |
[01:24:26]     |
[01:24:26] 605 |             assert_eq!(Cfg::parse(&mi), Ok(name_value_cfg("all", "done")));
[01:24:26]     |                                   ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:26]     |
[01:24:26]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:26] 
[01:24:26] error[E0308]: mismatched types
[01:24:26]    --> src/librustdoc/clean/cfg.rs:608:35
[01:24:26]     |
[01:24:26]     |
[01:24:26] 608 |             assert_eq!(Cfg::parse(&mi), Ok(word_cfg("a") & word_cfg("b")));
[01:24:26]     |                                   ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:26]     |
[01:24:26]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:26] 
[01:24:26] error[E0308]: mismatched types
[01:24:26]    --> src/librustdoc/clean/cfg.rs:611:35
[01:24:26]     |
[01:24:26]     |
[01:24:26] 611 |             assert_eq!(Cfg::parse(&mi), Ok(word_cfg("a") | word_cfg("b")));
[01:24:26]     |                                   ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:26]     |
[01:24:26]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:26] 
[01:24:26] error[E0308]: mismatched types
[01:24:26]    --> src/librustdoc/clean/cfg.rs:614:35
[01:24:26]     |
[01:24:26]     |
[01:24:26] 614 |             assert_eq!(Cfg::parse(&mi), Ok(!word_cfg("a")));
[01:24:26]     |                                   ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:26]     |
[01:24:26]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:26] 
[01:24:26] error[E0308]: mismatched types
[01:24:26]    --> src/librustdoc/clean/cfg.rs:622:35
[01:24:26]     |
[01:24:26]     |
[01:24:26] 622 |             assert_eq!(Cfg::parse(&mi), Ok(!(word_cfg("a") | (word_cfg("b") & word_cfg("c")))));
[01:24:26]     |                                   ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:26]     |
[01:24:26]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:26] 
[01:24:27] error[E0308]: mismatched types
[01:24:27]    --> src/librustdoc/clean/cfg.rs:625:35
[01:24:27]     |
[01:24:27]     |
[01:24:27] 625 |             assert_eq!(Cfg::parse(&mi), Ok(word_cfg("a") & word_cfg("b") & word_cfg("c")));
[01:24:27]     |                                   ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:27]     |
[01:24:27]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:27] 
[01:24:27] error[E0308]: mismatched types
[01:24:27]    --> src/librustdoc/clean/cfg.rs:637:32
[01:24:27]     |
[01:24:27]     |
[01:24:27] 637 |             assert!(Cfg::parse(&mi).is_err());
[01:24:27]     |                                ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:27]     |
[01:24:27]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:27] 
[01:24:27] error[E0308]: mismatched types
[01:24:27]    --> src/librustdoc/clean/cfg.rs:640:32
[01:24:27]     |
[01:24:27]     |
[01:24:27] 640 |             assert!(Cfg::parse(&mi).is_err());
[01:24:27]     |                                ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:27]     |
[01:24:27]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:27] 
[01:24:27] error[E0308]: mismatched types
[01:24:27]    --> src/librustdoc/clean/cfg.rs:643:32
[01:24:27]     |
[01:24:27]     |
[01:24:27] 643 |             assert!(Cfg::parse(&mi).is_err());
[01:24:27]     |                                ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:27]     |
[01:24:27]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:27] 
[01:24:28] error[E0308]: mismatched types
[01:24:28]    --> src/librustdoc/clean/cfg.rs:646:32
[01:24:28]     |
[01:24:28]     |
[01:24:28] 646 |             assert!(Cfg::parse(&mi).is_err());
[01:24:28]     |                                ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:28]     |
[01:24:28]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:28] 
[01:24:28] error[E0308]: mismatched types
[01:24:28]    --> src/librustdoc/clean/cfg.rs:652:32
[01:24:28]     |
[01:24:28]     |
[01:24:28] 652 |             assert!(Cfg::parse(&mi).is_err());
[01:24:28]     |                                ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:28]     |
[01:24:28]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:28] 
[01:24:28] error[E0308]: mismatched types
[01:24:28]    --> src/librustdoc/clean/cfg.rs:658:32
[01:24:28]     |
[01:24:28]     |
[01:24:28] 658 |             assert!(Cfg::parse(&mi).is_err());
[01:24:28]     |                                ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:28]     |
[01:24:28]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:28] 
[01:24:28] error[E0308]: mismatched types
[01:24:28]    --> src/librustdoc/clean/cfg.rs:663:32
[01:24:28]     |
[01:24:28]     |
[01:24:28] 663 |             assert!(Cfg::parse(&mi).is_err());
[01:24:28]     |                                ^^^ expected struct `rustc::hir::Path`, found struct `syntax::ast::Path`
[01:24:28]     |
[01:24:28]     = note: expected type `&syntax::ast::MetaItem<rustc::hir::Path>`
[01:24:28] 
[01:24:31] error: aborting due to 14 previous errors
[01:24:31] 
[01:24:31] For more information about this error, try `rustc --explain E0308`.
[01:24:31] For more information about this error, try `rustc --explain E0308`.
[01:24:31] error: Could not compile `rustdoc`.
[01:24:31] 
[01:24:31] To learn more, run the command again with --verbose.
[01:24:31] 
[01:24:31] 
[01:24:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:24:31] 
[01:24:31] 
[01:24:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:31] Build completed unsuccessfully in 0:37:57
[01:24:31] Build completed unsuccessfully in 0:37:57
[01:24:31] make: *** [check] Error 1
[01:24:31] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f8eb8a9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 22:40:59 UTC 2018
