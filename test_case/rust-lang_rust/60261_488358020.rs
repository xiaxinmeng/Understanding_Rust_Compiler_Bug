plain
travis_time:end:05792ba4:start=1556728809613319200,finish=1556728896204794452,duration=86591475252
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:08:41] .................................................................................................... 1200/5475
[01:08:43] .................................................................................................... 1300/5475
[01:08:46] .................................................................................................... 1400/5475
[01:08:49] .................................................................................................... 1500/5475
[01:08:51] ...........................................................................................F........ 1600/5475
[01:08:58] .................................................................................................... 1800/5475
[01:09:02] .................................................................................................... 1900/5475
[01:09:05] .................................................................................................... 2000/5475
[01:09:08] .............................................i...................................................... 2100/5475
---
[01:10:02] ..............................................ii...i..ii............................................ 3500/5475
[01:10:06] .................................................................................................... 3600/5475
[01:10:09] .................................................................................................... 3700/5475
[01:10:13] .....................................................ii............................................. 3800/5475
[01:10:15] ...........................................................................i.F...................... 3900/5475
[01:10:17] .............................................F...................................................... 4000/5475
[01:10:19] ...............F...................i.......F........................................................ 4100/5475
[01:10:32] .................................................................................................... 4300/5475
[01:10:37] .................................................................................................... 4400/5475
[01:10:40] .................................................................................................... 4500/5475
[01:10:44] .................................................................................................... 4600/5475
---
[01:11:16] 
[01:11:16] ---- [ui] ui/fmt/format-string-error-2.rs stdout ----
[01:11:16] diff of stderr:
[01:11:16] 
[01:11:16] 150    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 152 error: incorrect unicode escape sequence
[01:11:16] -   --> $DIR/format-string-error-2.rs:77:15
[01:11:16] +   --> $DIR/format-string-error-2.rs:77:20
[01:11:16] 154    |
[01:11:16] 154    |
[01:11:16] 155 LL |     println!("\x7B}\u8 {", 1);
[01:11:16] +    |                    ^^^
[01:11:16] 157 
[01:11:16] 157 
[01:11:16] 158 error: invalid format string: expected `'}'` but string was terminated
[01:11:16] 
[01:11:16] 
[01:11:16] The actual stderr differed from the expected stderr.
[01:11:16] The actual stderr differed from the expected stderr.
[01:11:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/format-string-error-2.stderr
[01:11:16] To update references, rerun the tests and pass the `--bless` flag
[01:11:16] To only update this specific test, also pass `--test-args fmt/format-string-error-2.rs`
[01:11:16] error: 1 errors occurred comparing output.
[01:11:16] status: exit code: 1
[01:11:16] status: exit code: 1
[01:11:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/auxiliary" "-A" "unused"
[01:11:16] ------------------------------------------
[01:11:16] 
[01:11:16] ------------------------------------------
[01:11:16] stderr:
[01:11:16] stderr:
[01:11:16] ------------------------------------------
[01:11:16] error: invalid format string: expected `'}'`, found `'a'`
[01:11:16]    |
[01:11:16] LL |     format!("{
[01:11:16]    |              - because of this opening brace
[01:11:16] LL |     a");
[01:11:16] LL |     a");
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'b'`
[01:11:16]    |
[01:11:16] LL |     format!("{ \
[01:11:16]    |              - because of this opening brace
[01:11:16] LL | 
[01:11:16] LL | 
[01:11:16] LL |     b");
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'\\'`
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     format!(r#"{ \
[01:11:16]    |                - ^ expected `}` in format string
[01:11:16]    |                because of this opening brace
[01:11:16]    |
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'\\'`
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     format!(r#"{ \n
[01:11:16]    |                - ^ expected `}` in format string
[01:11:16]    |                because of this opening brace
[01:11:16]    |
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'e'`
[01:11:16]    |
[01:11:16] LL |     format!("{ \n
[01:11:16]    |              - because of this opening brace
[01:11:16] LL | \n
[01:11:16] LL | \n
[01:11:16] LL |     e");
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'a'`
[01:11:16]    |
[01:11:16] LL |     {
[01:11:16]    |     - because of this opening brace
[01:11:16] LL |     a");
[01:11:16] LL |     a");
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'a'`
[01:11:16]    |
[01:11:16] LL |     {
[01:11:16]    |     - because of this opening brace
[01:11:16] LL |     a
[01:11:16] LL |     a
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'b'`
[01:11:16]    |
[01:11:16] LL |     { \
[01:11:16]    |     - because of this opening brace
[01:11:16] LL |         \
[01:11:16] LL |         \
[01:11:16] LL |     b");
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'b'`
[01:11:16]    |
[01:11:16] LL |     { \
[01:11:16]    |     - because of this opening brace
[01:11:16] LL |         \
[01:11:16] LL |         \
[01:11:16] LL |     b \
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'\\'`
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL | raw  { \
[01:11:16]    |      - ^ expected `}` in format string
[01:11:16]    |      because of this opening brace
[01:11:16]    |
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'\\'`
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL | raw  { \n
[01:11:16]    |      - ^ expected `}` in format string
[01:11:16]    |      because of this opening brace
[01:11:16]    |
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'e'`
[01:11:16]    |
[01:11:16] LL |   { \n
[01:11:16]    |   - because of this opening brace
[01:11:16] LL | \n
[01:11:16] LL | \n
[01:11:16] LL |     e");
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'`, found `'a'`
[01:11:16]    |
[01:11:16] LL |     {
[01:11:16]    |     - because of this opening brace
[01:11:16] LL |     asdf}
[01:11:16] LL |     asdf}
[01:11:16]    |     ^ expected `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: 1 positional argument in format string, but no arguments were given
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     println!("\t{}");
[01:11:16] 
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'` but string was terminated
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     println!("\x7B}\u{8} {", 1);
[01:11:16]    |                          -^ expected `'}'` in format string
[01:11:16]    |                          because of this opening brace
[01:11:16]    |
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] error: incorrect unicode escape sequence
[01:11:16]   --> /checkout/src/test/ui/fmt/format-string-error-2.rs:77:20
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     println!("\x7B}\u8 {", 1);
[01:11:16] 
[01:11:16] 
[01:11:16] error: invalid format string: expected `'}'` but string was terminated
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     println!("\x7B}\u8 {", 1);
[01:11:16]    |                          -^ expected `'}'` in format string
[01:11:16]    |                          because of this opening brace
[01:11:16]    |
[01:11:16]    |
[01:11:16]    = note: if you intended to print `{`, you can escape it using `{{`
[01:11:16] 
[01:11:16] error: invalid format string: unmatched `}` found
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     println!(r#"\x7B}\u{8} {"#, 1);
[01:11:16]    |                     ^ unmatched `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `}`, you can escape it using `}}`
[01:11:16] 
[01:11:16] error: invalid format string: unmatched `}` found
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     println!(r#"\x7B}\u8 {"#, 1);
[01:11:16]    |                     ^ unmatched `}` in format string
[01:11:16]    |
[01:11:16]    = note: if you intended to print `}`, you can escape it using `}}`
[01:11:16] error: aborting due to 19 previous errors
[01:11:16] 
[01:11:16] 
[01:11:16] ------------------------------------------
[01:11:16] ------------------------------------------
[01:11:16] 
[01:11:16] 
[01:11:16] ---- [ui] ui/parser/byte-string-literals.rs stdout ----
[01:11:16] 
[01:11:16] error: /checkout/src/test/ui/parser/byte-string-literals.rs:3: expected error not found: unknown byte escape
[01:11:16] 
[01:11:16] error: /checkout/src/test/ui/parser/byte-string-literals.rs:6: expected error not found: unknown byte escape
[01:11:16] 
[01:11:16] error: /checkout/src/test/ui/parser/byte-string-literals.rs:7: expected error not found: invalid character in numeric character escape: Z
[01:11:16] error: /checkout/src/test/ui/parser/byte-string-literals.rs:8: expected error not found: byte constant must be ASCII
[01:11:16] 
[01:11:16] error: 0 unexpected errors found, 4 expected errors not found
[01:11:16] status: exit code: 1
[01:11:16] status: exit code: 1
[01:11:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/auxiliary" "-A" "unused"
[01:11:16]     Error {
[01:11:16]         line_num: 3,
[01:11:16]         kind: Some(
[01:11:16]             Error,
[01:11:16]             Error,
[01:11:16]         ),
[01:11:16]         msg: "unknown byte escape",
[01:11:16]     Error {
[01:11:16]         line_num: 6,
[01:11:16]         kind: Some(
[01:11:16]             Error,
[01:11:16]             Error,
[01:11:16]         ),
[01:11:16]         msg: "unknown byte escape",
[01:11:16]     Error {
[01:11:16]         line_num: 7,
[01:11:16]         kind: Some(
[01:11:16]             Error,
[01:11:16]             Error,
[01:11:16]         ),
[01:11:16]         msg: "invalid character in numeric character escape: Z",
[01:11:16]     Error {
[01:11:16]         line_num: 8,
[01:11:16]         kind: Some(
[01:11:16]             Error,
[01:11:16]             Error,
[01:11:16]         ),
[01:11:16]         msg: "byte constant must be ASCII",
[01:11:16] ]
[01:11:16] 
[01:11:16] thread '[ui] ui/parser/byte-string-literals.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1402:13
[01:11:16] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:16] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:16] 
[01:11:16] ---- [ui] ui/parser/issue-23620-invalid-escapes.rs stdout ----
[01:11:16] diff of stderr:
[01:11:16] 
[01:11:16] 14   --> $DIR/issue-23620-invalid-escapes.rs:10:15
[01:11:16] 15    |
[01:11:16] 16 LL |     let _ = b'\u';
[01:11:16] -    |               ^^ incorrect unicode escape sequence
[01:11:16] -    |
[01:11:16] -    = help: format of unicode escape sequences is `\u{...}`
[01:11:16] - error: unicode escape sequences cannot be used as a byte or in a byte string
[01:11:16] -   --> $DIR/issue-23620-invalid-escapes.rs:10:15
[01:11:16] -    |
[01:11:16] -    |
[01:11:16] - LL |     let _ = b'\u';
[01:11:16] 26 
[01:11:16] 26 
[01:11:16] 27 error: numeric character escape is too short
[01:11:16] -   --> $DIR/issue-23620-invalid-escapes.rs:14:17
[01:11:16] +   --> $DIR/issue-23620-invalid-escapes.rs:14:15
[01:11:16] 29    |
[01:11:16] 29    |
[01:11:16] 30 LL |     let _ = b'\x5';
[01:11:16] +    |               ^^^
[01:11:16] 32 
[01:11:16] 32 
[01:11:16] 33 error: invalid character in numeric character escape: x
[01:11:16] 
[01:11:16] 
[01:11:16] 36 LL |     let _ = b'\xxy';
[01:11:16] 38 
[01:11:16] 38 
[01:11:16] - error: invalid character in numeric character escape: y
[01:11:16] -    |
[01:11:16] -    |
[01:11:16] - LL |     let _ = b'\xxy';
[01:11:16] - 
[01:11:16] - 
[01:11:16] 45 error: numeric character escape is too short
[01:11:16] +   --> $DIR/issue-23620-invalid-escapes.rs:21:14
[01:11:16] 47    |
[01:11:16] 48 LL |     let _ = '\x5';
[01:11:16] -    |                ^
[01:11:16] -    |                ^
[01:11:16] +    |              ^^^
[01:11:16] 50 
[01:11:16] 51 error: invalid character in numeric character escape: x
[01:11:16] 
[01:11:16] 
[01:11:16] 54 LL |     let _ = '\xxy';
[01:11:16] 56 
[01:11:16] 56 
[01:11:16] - error: invalid character in numeric character escape: y
[01:11:16] -    |
[01:11:16] -    |
[01:11:16] - LL |     let _ = '\xxy';
[01:11:16] - 
[01:11:16] 63 error: unicode escape sequences cannot be used as a byte or in a byte string
[01:11:16] 64   --> $DIR/issue-23620-invalid-escapes.rs:28:15
[01:11:16] 65    |
[01:11:16] 65    |
[01:11:16] 
[01:11:16] 76   --> $DIR/issue-23620-invalid-escapes.rs:28:28
[01:11:16] 77    |
[01:11:16] 78 LL |     let _ = b"\u{a4a4} \xf \u";
[01:11:16] -    |                            ^^ incorrect unicode escape sequence
[01:11:16] -    |
[01:11:16] -    = help: format of unicode escape sequences is `\u{...}`
[01:11:16] - error: unicode escape sequences cannot be used as a byte or in a byte string
[01:11:16] -   --> $DIR/issue-23620-invalid-escapes.rs:28:28
[01:11:16] -    |
[01:11:16] -    |
[01:11:16] - LL |     let _ = b"\u{a4a4} \xf \u";
[01:11:16] 88 
[01:11:16] 88 
[01:11:16] 89 error: invalid character in numeric character escape:  
[01:11:16] 
[01:11:16] 92 LL |     let _ = "\xf \u";
[01:11:16] 94 
[01:11:16] 94 
[01:11:16] - error: this form of character escape may only be used with characters in the range [\x00-\x7f]
[01:11:16] -    |
[01:11:16] -    |
[01:11:16] - LL |     let _ = "\xf \u";
[01:11:16] - 
[01:11:16] 101 error: incorrect unicode escape sequence
[01:11:16] 102   --> $DIR/issue-23620-invalid-escapes.rs:34:18
[01:11:16] 103    |
[01:11:16] 103    |
[01:11:16] 
[01:11:16] 104 LL |     let _ = "\xf \u";
[01:11:16] -    |                  ^^ incorrect unicode escape sequence
[01:11:16] -    |
[01:11:16] -    = help: format of unicode escape sequences is `\u{...}`
[01:11:16] 108 
[01:11:16] 109 error: incorrect unicode escape sequence
[01:11:16] 110   --> $DIR/issue-23620-invalid-escapes.rs:39:14
[01:11:16] 
[01:11:16] 
[01:11:16] 111    |
[01:11:16] 112 LL |     let _ = "\u8f";
[01:11:16] -    |              ^^--
[01:11:16] -    |              |
[01:11:16] -    |              help: format of unicode escape sequences uses braces: `\u{8f}`
[01:11:16] 116 
[01:11:16] - error: aborting due to 18 previous errors
[01:11:16] + error: aborting due to 13 previous errors
[01:11:16] 118 
[01:11:16] 118 
[01:11:16] 119 
[01:11:16] 
[01:11:16] 
[01:11:16] The actual stderr differed from the expected stderr.
[01:11:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/issue-23620-invalid-escapes.stderr
[01:11:16] To update references, rerun the tests and pass the `--bless` flag
[01:11:16] To only update this specific test, also pass `--test-args parser/issue-23620-invalid-escapes.rs`
[01:11:16] error: 1 errors occurred comparing output.
[01:11:16] status: exit code: 1
[01:11:16] status: exit code: 1
[01:11:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/auxiliary" "-A" "unused"
[01:11:16] ------------------------------------------
[01:11:16] 
[01:11:16] ------------------------------------------
[01:11:16] stderr:
[01:11:16] stderr:
[01:11:16] ------------------------------------------
[01:11:16] error: unicode escape sequences cannot be used as a byte or in a byte string
[01:11:16]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:4:15
[01:11:16]    |
[01:11:16] LL |     let _ = b"\u{a66e}";
[01:11:16] 
[01:11:16] error: unicode escape sequences cannot be used as a byte or in a byte string
[01:11:16]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:7:15
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = b'\u{a66e}';
[01:11:16] 
[01:11:16] error: incorrect unicode escape sequence
[01:11:16]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:10:15
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = b'\u';
[01:11:16] 
[01:11:16] 
[01:11:16] error: numeric character escape is too short
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = b'\x5';
[01:11:16] 
[01:11:16] 
[01:11:16] error: invalid character in numeric character escape: x
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = b'\xxy';
[01:11:16] 
[01:11:16] 
[01:11:16] error: numeric character escape is too short
[01:11:16]    |
[01:11:16] LL |     let _ = '\x5';
[01:11:16]    |              ^^^
[01:11:16] 
[01:11:16] 
[01:11:16] error: invalid character in numeric character escape: x
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = '\xxy';
[01:11:16] 
[01:11:16] error: unicode escape sequences cannot be used as a byte or in a byte string
[01:11:16]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:28:15
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = b"\u{a4a4} \xf \u";
[01:11:16] 
[01:11:16] 
[01:11:16] error: invalid character in numeric character escape:  
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = b"\u{a4a4} \xf \u";
[01:11:16] 
[01:11:16] error: incorrect unicode escape sequence
[01:11:16]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:28:28
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = b"\u{a4a4} \xf \u";
[01:11:16] 
[01:11:16] 
[01:11:16] error: invalid character in numeric character escape:  
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = "\xf \u";
[01:11:16] 
[01:11:16] error: incorrect unicode escape sequence
[01:11:16]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:34:18
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = "\xf \u";
[01:11:16] 
[01:11:16] error: incorrect unicode escape sequence
[01:11:16]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:39:14
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _ = "\u8f";
[01:11:16] 
[01:11:16] error: aborting due to 13 previous errors
[01:11:16] 
[01:11:16] 
---
[01:11:16] 31 error: character constant must be escaped: \r
[01:11:16] -   --> $DIR/lex-bare-cr-string-literal-doc-comment.rs:21:15
[01:11:16] +   --> $DIR/lex-bare-cr-string-literal-doc-comment.rs:21:18
[01:11:16] 33    |
[01:11:16] 34 LL |     let _s = "foo
bar";
[01:11:16] +    |                  ^
[01:11:16] 36 
[01:11:16] 36 
[01:11:16] 37 error: unknown character escape: \r
[01:11:16] 
[01:11:16] 
[01:11:16] The actual stderr differed from the expected stderr.
[01:11:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment/lex-bare-cr-string-literal-doc-comment.stderr
[01:11:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment/lex-bare-cr-string-literal-doc-comment.stderr
[01:11:16] To update references, rerun the tests and pass the `--bless` flag
[01:11:16] To only update this specific test, also pass `--test-args parser/lex-bare-cr-string-literal-doc-comment.rs`
[01:11:16] error: 1 errors occurred comparing output.
[01:11:16] status: exit code: 1
[01:11:16] status: exit code: 1
[01:11:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment/auxiliary" "-A" "unused"
[01:11:16] ------------------------------------------
[01:11:16] 
[01:11:16] ------------------------------------------
[01:11:16] stderr:
[01:11:16] stderr:
[01:11:16] ------------------------------------------
[01:11:16] error: bare CR not allowed in doc-comment
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL | /// doc comment with bare CR: '
[01:11:16]    |                                ^
[01:11:16] 
[01:11:16] 
[01:11:16] error: bare CR not allowed in block doc-comment
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL | /** block doc comment with bare CR: '
[01:11:16]    |                                      ^
[01:11:16] 
[01:11:16] 
[01:11:16] error: bare CR not allowed in doc-comment
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     //! doc comment with bare CR: '
[01:11:16]    |                                    ^
[01:11:16] 
[01:11:16] 
[01:11:16] error: bare CR not allowed in block doc-comment
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     /*! block doc comment with bare CR: '
[01:11:16]    |                                          ^
[01:11:16] 
[01:11:16] 
[01:11:16] error: bare CR not allowed in raw string, use \r instead
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _s = r"bar
foo"; //~ ERROR: bare CR not allowed in raw string
[01:11:16] 
[01:11:16] error: character constant must be escaped: \r
[01:11:16]   --> /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:21:18
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _s = "foo
bar"; //~ ERROR: bare CR not allowed in string
[01:11:16] 
[01:11:16] 
[01:11:16] error: unknown character escape: \r
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let _s = "foo\
bar"; //~ ERROR: unknown character escape: \r
[01:11:16]    |                   ^ unknown character escape
[01:11:16]    |
[01:11:16]    = help: this is an isolated carriage return; consider checking your editor and version control settings
[01:11:16] error: aborting due to 7 previous errors
[01:11:16] 
[01:11:16] 
[01:11:16] ------------------------------------------
[01:11:16] ------------------------------------------
[01:11:16] 
[01:11:16] 
[01:11:16] ---- [ui] ui/parser/new-unicode-escapes-2.rs stdout ----
[01:11:16] diff of stderr:
[01:11:16] 
[01:11:16] 2   --> $DIR/new-unicode-escapes-2.rs:2:14
[01:11:16] 3    |
[01:11:16] 4 LL |     let s = "\u{260311111111}";
[01:11:16] +    |              ^^^^^^^^^^
[01:11:16] 6 
[01:11:16] 7 error: aborting due to previous error
[01:11:16] 8 
[01:11:16] 8 
[01:11:16] 
[01:11:16] 
[01:11:16] The actual stderr differed from the expected stderr.
[01:11:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/new-unicode-escapes-2/new-unicode-escapes-2.stderr
[01:11:16] To update references, rerun the tests and pass the `--bless` flag
[01:11:16] To only update this specific test, also pass `--test-args parser/new-unicode-escapes-2.rs`
[01:11:16] error: 1 errors occurred comparing output.
[01:11:16] status: exit code: 1
[01:11:16] status: exit code: 1
[01:11:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/new-unicode-escapes-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/new-unicode-escapes-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/new-unicode-escapes-2/auxiliary" "-A" "unused"
[01:11:16] ------------------------------------------
[01:11:16] 
[01:11:16] ------------------------------------------
[01:11:16] stderr:
[01:11:16] stderr:
[01:11:16] ------------------------------------------
[01:11:16] error: overlong unicode escape (must have at most 6 hex digits)
[01:11:16]    |
[01:11:16]    |
[01:11:16] LL |     let s = "\u{260311111111}"; //~ ERROR overlong unicode escape (must have at most 6 hex digits)
[01:11:16] 
[01:11:16] error: aborting due to previous error
[01:11:16] 
[01:11:16] 
---
[01:11:16] 
[01:11:16] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:11:16] 
[01:11:16] 
[01:11:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:16] 
[01:11:16] 
[01:11:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:16] Build completed unsuccessfully in 0:04:16
[01:11:16] Build completed unsuccessfully in 0:04:16
[01:11:16] Makefile:48: recipe for target 'check' failed
[01:11:16] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04b91248
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 17:53:02 UTC 2019
