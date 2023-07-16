plain
travis_time:end:0e53d2f2:start=1560862519975254417,finish=1560862609096315831,duration=89121061414
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:13]    Compiling itertools v0.8.0
[00:06:14]    Compiling rustc-hash v1.0.1
[00:06:14]    Compiling serialize v0.0.0 (/checkout/src/libserialize)
[00:06:15]    Compiling backtrace v0.3.29
[00:06:15]    Compiling rustc_lexer v0.1.0 (/checkout/src/rustc_lexer)
[00:06:17]    Compiling humantime v1.2.0
[00:06:18]    Compiling rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:06:18]    Compiling ena v0.13.0
[00:06:18]    Compiling crossbeam-epoch v0.3.1
---
[00:28:32]    Compiling itertools v0.8.0
[00:28:33]    Compiling serialize v0.0.0 (/checkout/src/libserialize)
[00:28:34]    Compiling backtrace v0.3.29
[00:28:35]    Compiling chalk-macros v0.1.0
[00:28:35]    Compiling rustc_lexer v0.1.0 (/checkout/src/rustc_lexer)
[00:28:37]    Compiling ena v0.13.0
[00:28:38]    Compiling rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:28:38]    Compiling crossbeam-epoch v0.3.1
[00:28:40]    Compiling lock_api v0.1.3
---
[00:54:53] .................................................................................................... 700/5680
[00:54:57] .................................................................................................... 800/5680
[00:55:02] .................................................................................................... 900/5680
[00:55:07] ........................................i...........i............................................... 1000/5680
[00:55:10] ............................................................F........iiiii.......................... 1100/5680
[00:55:16] .................................................................................................... 1300/5680
[00:55:18] .................................................................................................... 1400/5680
[00:55:22] .................................................................................................... 1500/5680
[00:55:24] .................................................................................................... 1600/5680
---
[00:56:37] ...............................................................ii...i..ii........................... 3600/5680
[00:56:41] .................................................................................................... 3700/5680
[00:56:45] .................................................................................................... 3800/5680
[00:56:48] .........................................................................ii......................... 3900/5680
[00:56:51] ..............................................................................................iFF... 4000/5680
[00:56:53] .................................................................F.................................. 4100/5680
[00:56:55] ................................F.........................i........................................F 4200/5680
[00:56:58] FF..F.F......................................................F.F.................................... 4300/5680
[00:57:16] .................................................................................................... 4500/5680
[00:57:19] .................................................................................................... 4600/5680
[00:57:22] .................................................................................................... 4700/5680
[00:57:27] .................................................................................................... 4800/5680
---
[00:58:02] 
[00:58:02] 1 error: expected at least one digit in exponent
[00:58:02] +   --> $DIR/issue-49746-unicode-confusable-in-float-literal-expt.rs:1:1
[00:58:02] +    |
[00:58:02] + LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
[00:58:02] + 
[00:58:02] + 
[00:58:02] + error: unknown start of token: \u{2212}
[00:58:02] 3    |
[00:58:02] 3    |
[00:58:02] 4 LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
[00:58:02] 5    |                                                     ^
[00:58:02] 5    |                                                     ^
[00:58:02] 6 help: Unicode character '−' (Minus Sign) looks like '-' (Minus/Hyphen), but it is not
[00:58:02] 7    |
[00:58:02] - LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m³⋅kg⁻¹⋅s⁻²
[00:58:02] -    |                                                     ^
[00:58:02] + LL | -11; // m³⋅kg⁻¹⋅s⁻²
[00:58:02] 10 
[00:58:02] - error: aborting due to previous error
[00:58:02] + error: aborting due to 2 previous errors
[00:58:02] 12 
[00:58:02] 12 
[00:58:02] 13 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt/issue-49746-unicode-confusable-in-float-literal-expt.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: expected at least one digit in exponent
[00:58:02]   --> /checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs:1:1
[00:58:02]    |
[00:58:02] LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
[00:58:02] 
[00:58:02] error: unknown start of token: \u{2212}
[00:58:02]   --> /checkout/src/test/ui/did_you_mean/issue-49746-unicode-confusable-in-float-literal-expt.rs:1:53
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL | const UNIVERSAL_GRAVITATIONAL_CONSTANT: f64 = 6.674e−11; // m³⋅kg⁻¹⋅s⁻²
[00:58:02]    |                                                     ^
[00:58:02] help: Unicode character '−' (Minus Sign) looks like '-' (Minus/Hyphen), but it is not
[00:58:02]    |
[00:58:02] LL | -11; // m³⋅kg⁻¹⋅s⁻²
[00:58:02] 
[00:58:02] error: aborting due to 2 previous errors
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] 
[00:58:02] ---- [ui] ui/parser/byte-literals.rs stdout ----
[00:58:02] 
[00:58:02] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:58:02] status: exit code: 101
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/byte-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-literals/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unknown byte escape: f
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL | static FOO: u8 = b'\f';  //~ ERROR unknown byte escape
[00:58:02]    |                    ^ unknown byte escape
[00:58:02] 
[00:58:02] error: unknown byte escape: f
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     b'\f';  //~ ERROR unknown byte escape
[00:58:02]    |       ^ unknown byte escape
[00:58:02] 
[00:58:02] error: invalid character in numeric character escape: Z
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     b'\x0Z';  //~ ERROR invalid character in numeric character escape: Z
[00:58:02] 
[00:58:02] 
[00:58:02] error: byte constant must be escaped: \t
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     b'    ';  //~ ERROR byte constant must be escaped
[00:58:02] 
[00:58:02] error: byte constant must be escaped: '
[00:58:02]   --> /checkout/src/test/ui/parser/byte-literals.rs:12:6
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     b''';  //~ ERROR byte constant must be escaped
[00:58:02] 
[00:58:02] 
[00:58:02] thread 'rustc' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', src/libsyntax/source_map.rs:842:17
[00:58:02] error: aborting due to 5 previous errors
[00:58:02] 
[00:58:02] 
[00:58:02] error: internal compiler error: unexpected panic
[00:58:02] error: internal compiler error: unexpected panic
[00:58:02] 
[00:58:02] note: the compiler unexpectedly panicked. this is a bug.
[00:58:02] 
[00:58:02] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:02] 
[00:58:02] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:58:02] 
[00:58:02] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z continue-parse-after-error -C prefer-dynamic -C rpath -C debuginfo=0
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] ---- [ui] ui/parser/byte-string-literals.rs stdout ----
[00:58:02] 
[00:58:02] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:58:02] status: exit code: 101
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unknown byte escape: f
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL | static FOO: &'static [u8] = b"\f";  //~ ERROR unknown byte escape
[00:58:02]    |                               ^ unknown byte escape
[00:58:02] 
[00:58:02] error: unknown byte escape: f
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     b"\f";  //~ ERROR unknown byte escape
[00:58:02]    |       ^ unknown byte escape
[00:58:02] 
[00:58:02] error: invalid character in numeric character escape: Z
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     b"\x0Z";  //~ ERROR invalid character in numeric character escape: Z
[00:58:02] 
[00:58:02] 
[00:58:02] thread 'rustc' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', src/libsyntax/source_map.rs:842:17
[00:58:02] error: aborting due to 3 previous errors
[00:58:02] 
[00:58:02] 
[00:58:02] error: internal compiler error: unexpected panic
[00:58:02] error: internal compiler error: unexpected panic
[00:58:02] 
[00:58:02] note: the compiler unexpectedly panicked. this is a bug.
[00:58:02] 
[00:58:02] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:02] 
[00:58:02] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:58:02] 
[00:58:02] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z continue-parse-after-error -C prefer-dynamic -C rpath -C debuginfo=0
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] ---- [ui] ui/parser/issue-23620-invalid-escapes.rs stdout ----
[00:58:02] diff of stderr:
[00:58:02] 
[00:58:02] 1 error: unicode escape sequences cannot be used as a byte or in a byte string
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:4:15
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:4:14
[00:58:02] 3    |
[00:58:02] 4 LL |     let _ = b"\u{a66e}";
[00:58:02] +    |              ^^^^^^^^
[00:58:02] 6 
[00:58:02] 7 error: unicode escape sequences cannot be used as a byte or in a byte string
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:7:15
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:7:15
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:7:14
[00:58:02] 9    |
[00:58:02] 10 LL |     let _ = b'\u{a66e}';
[00:58:02] +    |              ^^^^^^^^
[00:58:02] 12 
[00:58:02] 13 error: incorrect unicode escape sequence
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:10:15
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:10:15
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:10:14
[00:58:02] 15    |
[00:58:02] 16 LL |     let _ = b'\u';
[00:58:02] -    |               ^^ incorrect unicode escape sequence
[00:58:02] +    |              ^^ incorrect unicode escape sequence
[00:58:02] 18    |
[00:58:02] 19    = help: format of unicode escape sequences is `\u{...}`
[00:58:02] 
[00:58:02] 
[00:58:02] 21 error: numeric character escape is too short
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:13:14
[00:58:02] 23    |
[00:58:02] 23    |
[00:58:02] 24 LL |     let _ = b'\x5';
[00:58:02] +    |              ^^^
[00:58:02] 26 
[00:58:02] 26 
[00:58:02] 27 error: invalid character in numeric character escape: x
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:16:16
[00:58:02] 29    |
[00:58:02] 29    |
[00:58:02] 30 LL |     let _ = b'\xxy';
[00:58:02] +    |                ^
[00:58:02] 32 
[00:58:02] 32 
[00:58:02] 33 error: numeric character escape is too short
[00:58:02] 
[00:58:02] 43    |                ^
[00:58:02] 44 
[00:58:02] 45 error: unicode escape sequences cannot be used as a byte or in a byte string
[00:58:02] 45 error: unicode escape sequences cannot be used as a byte or in a byte string
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:25:15
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:25:14
[00:58:02] 47    |
[00:58:02] 48 LL |     let _ = b"\u{a4a4} \xf \u";
[00:58:02] +    |              ^^^^^^^^
[00:58:02] 50 
[00:58:02] 50 
[00:58:02] 51 error: invalid character in numeric character escape:  
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:25:26
[00:58:02] 53    |
[00:58:02] 53    |
[00:58:02] 54 LL |     let _ = b"\u{a4a4} \xf \u";
[00:58:02] +    |                          ^
[00:58:02] 56 
[00:58:02] 57 error: incorrect unicode escape sequence
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:25:28
[00:58:02] -   --> $DIR/issue-23620-invalid-escapes.rs:25:28
[00:58:02] +   --> $DIR/issue-23620-invalid-escapes.rs:25:27
[00:58:02] 59    |
[00:58:02] 60 LL |     let _ = b"\u{a4a4} \xf \u";
[00:58:02] -    |                            ^^ incorrect unicode escape sequence
[00:58:02] +    |                           ^^ incorrect unicode escape sequence
[00:58:02] 62    |
[00:58:02] 63    = help: format of unicode escape sequences is `\u{...}`
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/issue-23620-invalid-escapes.stderr
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/issue-23620-invalid-escapes.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/issue-23620-invalid-escapes.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unicode escape sequences cannot be used as a byte or in a byte string
[00:58:02]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:4:14
[00:58:02]    |
[00:58:02] LL |     let _ = b"\u{a66e}";
[00:58:02] 
[00:58:02] error: unicode escape sequences cannot be used as a byte or in a byte string
[00:58:02]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:7:14
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = b'\u{a66e}';
[00:58:02] 
[00:58:02] error: incorrect unicode escape sequence
[00:58:02]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:10:14
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = b'\u';
[00:58:02]    |              ^^ incorrect unicode escape sequence
[00:58:02]    |
[00:58:02]    = help: format of unicode escape sequences is `\u{...}`
[00:58:02] 
[00:58:02] error: numeric character escape is too short
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = b'\x5';
[00:58:02] 
[00:58:02] 
[00:58:02] error: invalid character in numeric character escape: x
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = b'\xxy';
[00:58:02] 
[00:58:02] 
[00:58:02] error: numeric character escape is too short
[00:58:02]    |
[00:58:02] LL |     let _ = '\x5';
[00:58:02]    |              ^^^
[00:58:02] 
[00:58:02] 
[00:58:02] error: invalid character in numeric character escape: x
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = '\xxy';
[00:58:02] 
[00:58:02] error: unicode escape sequences cannot be used as a byte or in a byte string
[00:58:02]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:25:14
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = b"\u{a4a4} \xf \u";
[00:58:02] 
[00:58:02] 
[00:58:02] error: invalid character in numeric character escape:  
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = b"\u{a4a4} \xf \u";
[00:58:02] 
[00:58:02] error: incorrect unicode escape sequence
[00:58:02]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:25:27
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = b"\u{a4a4} \xf \u";
[00:58:02]    |                           ^^ incorrect unicode escape sequence
[00:58:02]    |
[00:58:02]    = help: format of unicode escape sequences is `\u{...}`
[00:58:02] 
[00:58:02] error: invalid character in numeric character escape:  
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = "\xf \u";
[00:58:02] 
[00:58:02] error: incorrect unicode escape sequence
[00:58:02]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:30:18
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = "\xf \u";
[00:58:02]    |                  ^^ incorrect unicode escape sequence
[00:58:02]    |
[00:58:02]    = help: format of unicode escape sequences is `\u{...}`
[00:58:02] error: incorrect unicode escape sequence
[00:58:02]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:34:14
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     let _ = "\u8f";
[00:58:02]    |              ^^--
[00:58:02]    |                |
[00:58:02]    |                help: format of unicode escape sequences uses braces: `\u{8f}`
[00:58:02] error: aborting due to 13 previous errors
[00:58:02] 
[00:58:02] 
[00:58:02] ------------------------------------------
---
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bad-numeric-literals/lex-bad-numeric-literals.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/lex-bad-numeric-literals.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/lex-bad-numeric-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bad-numeric-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bad-numeric-literals/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:2:5
[00:58:02]    |
[00:58:02] LL |     0o1.0; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:4:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o3.0f32; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:5:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o4e4; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:6:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o5.0e5; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:7:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o6e6f32; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:8:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o7.0e7f64; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: hexadecimal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:9:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0x8.0e+9; //~ ERROR: hexadecimal float literal is not supported
[00:58:02] 
[00:58:02] error: hexadecimal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:10:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0x9.0e-9; //~ ERROR: hexadecimal float literal is not supported
[00:58:02] 
[00:58:02] error: no valid digits found for number
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:11:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o; //~ ERROR: no valid digits
[00:58:02] 
[00:58:02] error: expected at least one digit in exponent
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:1:1
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL | / fn main() {
[00:58:02] LL | |     0o1.0; //~ ERROR: octal float literal is not supported
[00:58:02] LL | |     0o2f32; //~ ERROR: octal float literal is not supported
[00:58:02] LL | |     0o3.0f32; //~ ERROR: octal float literal is not supported
[00:58:02] ...  |
[00:58:02] LL | |     0o; //~ ERROR: no valid digits
[00:58:02] LL | |     1e+; //~ ERROR: expected at least one digit in exponent
[00:58:02] 
[00:58:02] error: hexadecimal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:13:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0x539.0; //~ ERROR: hexadecimal float literal is not supported
[00:58:02] 
[00:58:02] error: no valid digits found for number
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:18:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0x; //~ ERROR: no valid digits
[00:58:02] 
[00:58:02] error: no valid digits found for number
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:19:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0xu32; //~ ERROR: no valid digits
[00:58:02] 
[00:58:02] error: no valid digits found for number
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:20:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0ou32; //~ ERROR: no valid digits
[00:58:02] 
[00:58:02] error: no valid digits found for number
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:21:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0bu32; //~ ERROR: no valid digits
[00:58:02] 
[00:58:02] error: no valid digits found for number
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:22:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0b; //~ ERROR: no valid digits
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:24:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o123.456; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: binary float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:26:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0b111.101; //~ ERROR: binary float literal is not supported
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:3:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0o2f32; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: integer literal is too large
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:14:5
[00:58:02]    |
---
[00:58:02] 
[00:58:02] error: octal float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:23:5
[00:58:02]    |
[00:58:02] LL |     0o123f64; //~ ERROR: octal float literal is not supported
[00:58:02] 
[00:58:02] error: binary float literal is not supported
[00:58:02]   --> /checkout/src/test/ui/parser/lex-bad-numeric-literals.rs:25:5
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     0b101f64; //~ ERROR: binary float literal is not supported
[00:58:02] 
[00:58:02] error: aborting due to 23 previous errors
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] 
[00:58:02] ---- [ui] ui/parser/raw-byte-string-eof.rs stdout ----
[00:58:02] diff of stderr:
[00:58:02] 
[00:58:02] - error: unterminated raw string
[00:58:02] + error: unterminated raw byte string
[00:58:02] 3    |
[00:58:02] 3    |
[00:58:02] - LL |     br##"a"#;
[00:58:02] -    |      ^ unterminated raw string
[00:58:02] -    |
[00:58:02] -    = note: this raw string should be terminated with `"##`
[00:58:02] + LL |       br##"a"#;
[00:58:02] + LL | | }
[00:58:02] +    | |__^
[00:58:02] 8 
[00:58:02] 9 error: aborting due to previous error
[00:58:02] 9 error: aborting due to previous error
[00:58:02] 10 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-eof/raw-byte-string-eof.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/raw-byte-string-eof.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw-byte-string-eof.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-eof" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-eof/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unterminated raw byte string
[00:58:02]   --> /checkout/src/test/ui/parser/raw-byte-string-eof.rs:2:6
[00:58:02]    |
[00:58:02] LL |       br##"a"#;  //~ unterminated raw string
[00:58:02] LL | | }
[00:58:02]    | |__^
[00:58:02] 
[00:58:02] error: aborting due to previous error
---
[00:58:02] 
[00:58:02] 10 LL |     br"é";
[00:58:02] 11    |        ^
[00:58:02] 12 
[00:58:02] - error: found invalid character; only `#` is allowed in raw string delimitation: ~
[00:58:02] + error: unterminated raw byte string
[00:58:02] 15    |
[00:58:02] 15    |
[00:58:02] 16 LL |     br##~"a"~##;
[00:58:02] -    |      ^^^
[00:58:02] +    |      ^^^^
[00:58:02] 18 
[00:58:02] 19 error: aborting due to 3 previous errors
[00:58:02] 19 error: aborting due to 3 previous errors
[00:58:02] 20 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals/raw-byte-string-literals.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/raw-byte-string-literals.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw-byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: bare CR not allowed in raw string
[00:58:02]    |
[00:58:02] LL |     br"a
[00:58:02] LL |     br"a
"; //~ ERROR bare CR not allowed in raw string
[00:58:02] 
[00:58:02] error: raw byte string must be ASCII
[00:58:02]   --> /checkout/src/test/ui/parser/raw-byte-string-literals.rs:5:8
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     br"é";  //~ ERROR raw byte string must be ASCII
[00:58:02] 
[00:58:02] error: unterminated raw byte string
[00:58:02]   --> /checkout/src/test/ui/parser/raw-byte-string-literals.rs:6:6
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     br##~"a"~##;  //~ ERROR only `#` is allowed in raw string delimitation
[00:58:02] 
[00:58:02] error: aborting due to 3 previous errors
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] 
[00:58:02] ---- [ui] ui/parser/raw-str-delim.rs stdout ----
[00:58:02] diff of stderr:
[00:58:02] 
[00:58:02] - error: found invalid character; only `#` is allowed in raw string delimitation: ~
[00:58:02] + error: unterminated raw string
[00:58:02] 3    |
[00:58:02] 3    |
[00:58:02] 4 LL |     r#~"#"~#
[00:58:02] -    |     ^^
[00:58:02] +    |     ^^^
[00:58:02] 6 
[00:58:02] 7 error: aborting due to previous error
[00:58:02] 7 error: aborting due to previous error
[00:58:02] 8 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-str-delim/raw-str-delim.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/raw-str-delim.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw-str-delim.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-str-delim" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-str-delim/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unterminated raw string
[00:58:02]   --> /checkout/src/test/ui/parser/raw-str-delim.rs:2:5
[00:58:02]    |
[00:58:02] LL |     r#~"#"~# //~ ERROR found invalid character; only `#` is allowed in raw string delimitation
[00:58:02] 
[00:58:02] error: aborting due to previous error
[00:58:02] 
[00:58:02] 
---
[00:58:02] 
[00:58:02] 1 error: unterminated raw string
[00:58:02] 2   --> $DIR/raw-str-unterminated.rs:2:5
[00:58:02] 3    |
[00:58:02] - LL |     r#" string literal goes on
[00:58:02] -    |     ^ unterminated raw string
[00:58:02] -    |
[00:58:02] -    = note: this raw string should be terminated with `"#`
[00:58:02] + LL | /     r#" string literal goes on
[00:58:02] + LL | |         and on
[00:58:02] + LL | |
[00:58:02] 8 
[00:58:02] 9 error: aborting due to previous error
[00:58:02] 10 
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-str-unterminated/raw-str-unterminated.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/raw-str-unterminated.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw-str-unterminated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-str-unterminated" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-str-unterminated/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unterminated raw string
[00:58:02]   --> /checkout/src/test/ui/parser/raw-str-unterminated.rs:2:5
[00:58:02]    |
[00:58:02] LL | /     r#" string literal goes on
[00:58:02] LL | |         and on
[00:58:02] LL | |     //~^^ ERROR unterminated raw string
[00:58:02] 
[00:58:02] error: aborting due to previous error
[00:58:02] 
[00:58:02] 
---
[00:58:02] 
[00:58:02] 1 error: unterminated raw string
[00:58:02] 2   --> $DIR/raw_string.rs:2:13
[00:58:02] 3    |
[00:58:02] - LL |     let x = r##"lol"#;
[00:58:02] -    |             ^ unterminated raw string
[00:58:02] -    |
[00:58:02] -    = note: this raw string should be terminated with `"##`
[00:58:02] + LL |       let x = r##"lol"#;
[00:58:02] + LL | |
[00:58:02] + LL | | }
[00:58:02] +    | |__^
[00:58:02] 8 
[00:58:02] 8 
[00:58:02] 9 error: aborting due to previous error
[00:58:02] 10 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw_string/raw_string.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/raw/raw_string.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw/raw_string.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw_string" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw_string/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unterminated raw string
[00:58:02]   --> /checkout/src/test/ui/parser/raw/raw_string.rs:2:13
[00:58:02]    |
[00:58:02] LL |       let x = r##"lol"#;
[00:58:02]    |  _____________^
[00:58:02] LL | |     //~^ ERROR unterminated raw string
[00:58:02]    | |__^
[00:58:02] 
[00:58:02] error: aborting due to previous error
[00:58:02] 
---
[00:58:02] ---- [ui] ui/parser/unicode-chars.rs stdout ----
[00:58:02] diff of stderr:
[00:58:02] 
[00:58:02] 5    |              ^
[00:58:02] 6 help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
[00:58:02] - LL |     let y = 0;
[00:58:02] -    |              ^
[00:58:02] + LL | ;
[00:58:02] +    |
[00:58:02] +    |
[00:58:02] 10 
[00:58:02] 11 error: aborting due to previous error
[00:58:02] 12 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars/unicode-chars.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/unicode-chars.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unknown start of token: \u{37e}
[00:58:02]   --> /checkout/src/test/ui/parser/unicode-chars.rs:2:14
[00:58:02]    |
[00:58:02] LL |     let y = 0;
[00:58:02]    |              ^
[00:58:02] help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
[00:58:02] LL | ;
[00:58:02]    |
[00:58:02] 
[00:58:02] error: aborting due to previous error
---
[00:58:02] ---- [ui] ui/parser/unicode-quote-chars.rs stdout ----
[00:58:02] diff of stderr:
[00:58:02] 
[00:58:02] 3    |
[00:58:02] 4 LL |     println!(“hello world”);
[00:58:02] 5    |              ^
[00:58:02] - help: Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
[00:58:02] + help: Unicode character '“' (Left Double Quotation Mark) looks like '"' (Quotation Mark), but it is not
[00:58:02] - LL |     println!("hello world");
[00:58:02] -    |              ^^^^^^^^^^^^^
[00:58:02] -    |              ^^^^^^^^^^^^^
[00:58:02] + LL | "hello world”);
[00:58:02] 10 
[00:58:02] 11 error: aborting due to previous error
[00:58:02] 12 
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] The actual stderr differed from the expected stderr.
[00:58:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/unicode-quote-chars.stderr
[00:58:02] To update references, rerun the tests and pass the `--bless` flag
[00:58:02] To only update this specific test, also pass `--test-args parser/unicode-quote-chars.rs`
[00:58:02] error: 1 errors occurred comparing output.
[00:58:02] status: exit code: 1
[00:58:02] status: exit code: 1
[00:58:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-quote-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/auxiliary" "-A" "unused"
[00:58:02] ------------------------------------------
[00:58:02] 
[00:58:02] ------------------------------------------
[00:58:02] stderr:
[00:58:02] stderr:
[00:58:02] ------------------------------------------
[00:58:02] error: unknown start of token: \u{201c}
[00:58:02]    |
[00:58:02]    |
[00:58:02] LL |     println!(“hello world”);
[00:58:02]    |              ^
[00:58:02] help: Unicode character '“' (Left Double Quotation Mark) looks like '"' (Quotation Mark), but it is not
[00:58:02]    |
[00:58:02] LL | "hello world”);
[00:58:02] 
[00:58:02] error: aborting due to previous error
[00:58:02] 
[00:58:02] 
---
[00:58:02] test result: FAILED. 5647 passed; 12 failed; 21 ignored; 0 measured; 0 filtered out
[00:58:02] 
[00:58:02] 
[00:58:02] 
[00:58:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:02] 
[00:58:02] 
[00:58:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:02] Build completed unsuccessfully in 0:53:20
---
travis_time:end:0a2432bf:start=1560866102270464524,finish=1560866102275439773,duration=4975249
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01086ab4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:035e5a20
travis_time:start:035e5a20
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a593be8
$ dmesg | grep -i kill
