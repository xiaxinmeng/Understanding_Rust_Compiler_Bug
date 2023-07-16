plain
travis_time:end:07159332:start=1559916312078052667,finish=1559916314325828546,duration=2247775879
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:11] 
[01:06:11] running 144 tests
[01:06:14] i..iii.....iii..iiii.....i......................i...i................i.....i..........ii.i..i..i.ii. 100/144
[01:06:15] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:06:15] 
[01:06:15]  finished in 4.695
[01:06:15] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:18] 
[01:06:18] running 9 tests
[01:06:18] iiiiiiiii
[01:06:18] 
[01:06:18]  finished in 0.158
[01:06:18] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:34] 
[01:06:34] running 122 tests
[01:07:00] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:05] .i.i......iii.i.....ii
[01:07:05] 
[01:07:05]  finished in 30.738
[01:07:05] travis_fold:end:test_debuginfo

---
[01:31:47]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:663:52
[01:31:49]     |
[01:31:49] 663 |                 { None, "", None, None, None, "!", (0, 2), });
[01:31:49]     |                                                    ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:665:52
[01:31:49]     |
[01:31:49]     |
[01:31:49] 665 |                 { None, "", None, None, None, "c", (0, 2), });
[01:31:49]     |                                                    ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:667:52
[01:31:49]     |
[01:31:49]     |
[01:31:49] 667 |                 { None, "", None, None, None, "s", (0, 2), });
[01:31:49]     |                                                    ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:669:64
[01:31:49]     |
[01:31:49]     |
[01:31:49] 669 |                 { None, "0", Some(N::Num(6)), None, None, "d", (0, 4), });
[01:31:49]     |                                                                ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:671:74
[01:31:49]     |
[01:31:49]     |
[01:31:49] 671 |                 { None, "", Some(N::Num(4)), Some(N::Num(2)), None, "f", (0, 5), });
[01:31:49]     |                                                                          ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:673:53
[01:31:49]     |
[01:31:49]     |
[01:31:49] 673 |                 { None, "#", None, None, None, "x", (0, 3), });
[01:31:49]     |                                                     ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:675:65
[01:31:49]     |
[01:31:49]     |
[01:31:49] 675 |                 { None, "-", Some(N::Num(10)), None, None, "s", (0, 5), });
[01:31:49]     |                                                                 ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:677:61
[01:31:49]     |
[01:31:49]     |
[01:31:49] 677 |                 { None, "", Some(N::Next), None, None, "s", (0, 3), });
[01:31:49]     |                                                             ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:679:74
[01:31:49]     |
[01:31:49]     |
[01:31:49] 679 |                 { None, "-", Some(N::Num(10)), Some(N::Next), None, "s", (0, 7), });
[01:31:49]     |                                                                          ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:681:71
[01:31:49]     |
[01:31:49]     |
[01:31:49] 681 |                 { None, "-", Some(N::Next), Some(N::Next), None, "s", (0, 6), });
[01:31:49]     |                                                                       ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:683:63
[01:31:49]     |
[01:31:49]     |
[01:31:49] 683 |                 { None, "", None, Some(N::Num(6)), None, "i", (0, 4), });
[01:31:49]     |                                                               ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:685:53
[01:31:49]     |
[01:31:49]     |
[01:31:49] 685 |                 { None, "+", None, None, None, "i", (0, 3), });
[01:31:49]     |                                                     ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:687:64
[01:31:49]     |
[01:31:49]     |
[01:31:49] 687 |                 { None, "0", Some(N::Num(8)), None, None, "X", (0, 4), });
[01:31:49]     |                                                                ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:689:57
[01:31:49]     |
[01:31:49]     |
[01:31:49] 689 |                 { None, "", None, None, Some("l"), "u", (0, 3), });
[01:31:49]     |                                                         ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:691:57
[01:31:49]     |
[01:31:49]     |
[01:31:49] 691 |                 { None, "", None, None, Some("I"), "u", (0, 3), });
[01:31:49]     |                                                         ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:49] error[E0308]: mismatched types
[01:31:49]    --> src/libsyntax_ext/format_foreign.rs:693:59
[01:31:49]     |
[01:31:49]     |
[01:31:49] 693 |                 { None, "", None, None, Some("I32"), "u", (0, 5), });
[01:31:49]     |                                                           ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:49]     |
[01:31:49]     = note: expected type `syntax_pos::InnerSpan`
[01:31:49]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:695:59
[01:31:50]     |
[01:31:50]     |
[01:31:50] 695 |                 { None, "", None, None, Some("I64"), "u", (0, 5), });
[01:31:50]     |                                                           ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:697:53
[01:31:50]     |
[01:31:50]     |
[01:31:50] 697 |                 { None, "'", None, None, None, "d", (0, 3), });
[01:31:50]     |                                                     ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:699:64
[01:31:50]     |
[01:31:50]     |
[01:31:50] 699 |                 { None, "", Some(N::Num(10)), None, None, "s", (0, 4), });
[01:31:50]     |                                                                ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:701:77
[01:31:50]     |
[01:31:50]     |
[01:31:50] 701 |                 { None, "-", Some(N::Num(10)), Some(N::Num(10)), None, "s", (0, 8), });
[01:31:50]     |                                                                             ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:703:55
[01:31:50]     |
[01:31:50]     |
[01:31:50] 703 |                 { Some(1), "", None, None, None, "d", (0, 4), });
[01:31:50]     |                                                       ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:705:66
[01:31:50]     |
[01:31:50]     |
[01:31:50] 705 |                 { Some(2), "", None, Some(N::Arg(3)), None, "d", (0, 8), });
[01:31:50]     |                                                                  ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:707:77
[01:31:50]     |
[01:31:50]     |
[01:31:50] 707 |                 { Some(1), "", Some(N::Arg(2)), Some(N::Arg(3)), None, "d", (0, 11), });
[01:31:50]     |                                                                             ^^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0308]: mismatched types
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:709:69
[01:31:50]     |
[01:31:50]     |
[01:31:50] 709 |                 { None, "-", Some(N::Num(8)), None, Some("l"), "d", (0, 5), });
[01:31:50]     |                                                                     ^^^^^^ expected struct `syntax_pos::InnerSpan`, found tuple
[01:31:50]     |
[01:31:50]     = note: expected type `syntax_pos::InnerSpan`
[01:31:50]                found type `({integer}, {integer})`
[01:31:50] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:715:32
[01:31:50]     |
[01:31:50]     |
[01:31:50] 267 |     pub fn iter_subs(s: &str, start_pos: usize) -> Substitutions<'_> {
[01:31:50]     |     ---------------------------------------------------------------- defined here
[01:31:50] ...
[01:31:50] 715 |             let subs: Vec<_> = iter_subs(s).map(|sub| sub.translate()).collect();
[01:31:50] 
[01:31:50] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:31:50]    --> src/libsyntax_ext/format_foreign.rs:945:32
[01:31:50]     |
[01:31:50]     |
[01:31:50] 809 |     pub fn iter_subs(s: &str, start_pos: usize) -> Substitutions<'_> {
[01:31:50]     |     ---------------------------------------------------------------- defined here
[01:31:50] ...
[01:31:50] 945 |             let subs: Vec<_> = iter_subs(s).map(|sub| sub.translate()).collect();
[01:31:50] 
[01:31:50] error: aborting due to 26 previous errors
[01:31:50] 
[01:31:50] Some errors have detailed explanations: E0061, E0308.
[01:31:50] Some errors have detailed explanations: E0061, E0308.
[01:31:50] For more information about an error, try `rustc --explain E0061`.
[01:31:50] error: Could not compile `syntax_ext`.
[01:31:50] 
[01:31:50] To learn more, run the command again with --verbose.
[01:31:50] 
[01:31:50] 
[01:31:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax_ext" "--" "--quiet"
[01:31:50] 
[01:31:50] 
[01:31:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:50] Build completed unsuccessfully in 1:26:43
