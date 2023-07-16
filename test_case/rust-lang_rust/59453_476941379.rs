plain
travis_time:end:202db5b6:start=1553649423888077997,finish=1553649497530531287,duration=73642453290
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:12:17] ............................................................................................ii...... 3800/5499
[01:12:20] .................................................................................................... 3900/5499
[01:12:22] ..........i......................................................................................... 4000/5499
[01:12:24] ....................................................................i............................... 4100/5499
[01:12:27] ......................F............................................................................. 4200/5499
[01:12:42] .................................................................................................... 4400/5499
[01:12:45] .................................................................................................... 4500/5499
[01:12:48] .................................................................................................... 4600/5499
[01:12:53] ................................i................................................................... 4700/5499
---
[01:13:22] ---- [ui] ui/parser/recover-from-bad-variant.rs stdout ----
[01:13:22] diff of stderr:
[01:13:22] 
[01:13:22] 3    |
[01:13:22] 4 LL |     let x = Enum::Foo(a: 3, b: 4);
[01:13:22] 5    |                          ^ expecting a type here because of type ascription
[01:13:22] +    |
[01:13:22] +    = note: type ascription is a nightly-only feature that lets you annotate an expression with a type: `<expr>: <type>`
[01:13:22] + note: this expression expects an ascribed type after the colon
[01:13:22] +   --> $DIR/recover-from-bad-variant.rs:7:23
[01:13:22] +    |
[01:13:22] + LL |     let x = Enum::Foo(a: 3, b: 4);
[01:13:22] +    |                       ^
[01:13:22] +    = help: this might be indicative of a syntax error elsewhere
[01:13:22] 7 error[E0532]: expected tuple struct/variant, found struct variant `Enum::Foo`
[01:13:22] 8   --> $DIR/recover-from-bad-variant.rs:10:9
[01:13:22] 
[01:13:22] 
[01:13:22] 
[01:13:22] The actual stderr differed from the expected stderr.
[01:13:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-bad-variant/recover-from-bad-variant.stderr
[01:13:22] To update references, rerun the tests and pass the `--bless` flag
[01:13:22] To only update this specific test, also pass `--test-args parser/recover-from-bad-variant.rs`
[01:13:22] error: 1 errors occurred comparing output.
[01:13:22] status: exit code: 1
[01:13:22] status: exit code: 1
[01:13:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-from-bad-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-bad-variant/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-bad-variant/auxiliary" "-A" "unused"
[01:13:22] ------------------------------------------
[01:13:22] 
[01:13:22] ------------------------------------------
[01:13:22] stderr:
[01:13:22] stderr:
[01:13:22] ------------------------------------------
[01:13:22] {"message":"expected type, found `3`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/recover-from-bad-variant.rs","byte_start":107,"byte_end":108,"line_start":7,"line_end":7,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    let x = Enum::Foo(a: 3, b: 4);","highlight_start":26,"highlight_end":27}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"type ascription is a nightly-only feature that lets you annotate an expression with a type: `<expr>: <type>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this expression expects an ascribed type after the colon","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/parser/recover-from-bad-variant.rs","byte_start":104,"byte_end":105,"line_start":7,"line_end":7,"column_start":23,"column_end":24,"is_primary":true,"text":[{"text":"    let x = Enum::Foo(a: 3, b: 4);","highlight_start":23,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this might be indicative of a syntax error elsewhere","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: expected type, found `3`\n  --> /checkout/src/test/ui/parser/recover-from-bad-variant.rs:7:26\n   |\nLL |     let x = Enum::Foo(a: 3, b: 4);\n   |                          ^ expecting a type here because of type ascription\n   |\n   = note: type ascription is a nightly-only feature that lets you annotate an expression with a type: `<expr>: <type>`\nnote: this expression expects an ascribed type after the colon\n  --> /checkout/src/test/ui/parser/recover-from-bad-variant.rs:7:23\n   |\nLL |     let x = Enum::Foo(a: 3, b: 4);\n   |                       ^\n   = help: this might be indicative of a syntax error elsewhere\n\n"}
[01:13:22] {"message":"expected tuple struct/variant, found struct variant `Enum::Foo`","code":{"code":"E0532","explanation":"\nPattern arm did not match expected kind.\n\nErroneous code example:\n\n