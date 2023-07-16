plain
travis_time:end:0362c424:start=1552444318609030203,finish=1552444411233174807,duration=92624144604
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:14:04] ..................................................................................................i. 900/5464
[01:14:10] ..............i..................................................................................... 1000/5464
[01:14:14] ...........................iiiii.................................................................... 1100/5464
[01:14:17] .................................................................................................... 1200/5464
[01:14:20] ...........................................................................................F........ 1300/5464
[01:14:26] .................................................................................................... 1500/5464
[01:14:29] .................................................................................................... 1600/5464
[01:14:33] ................................i................................................................... 1700/5464
[01:14:37] .................................................................................................... 1800/5464
[01:14:37] .................................................................................................... 1800/5464
[01:14:41] .................................................................................................... 1900/5464
[01:14:45] .................................................................................................... 2000/5464
[01:14:49] .......................................................................i............................ 2100/5464
[01:14:53] .................................................................................................... 2200/5464
[01:14:58] ..............................................................................F..................... 2300/5464
[01:15:07] .................................................................................................... 2500/5464
[01:15:11] .................................................................................................... 2600/5464
[01:15:15] .................................................................................................... 2700/5464
[01:15:15] .................................................................................................... 2700/5464
[01:15:20] .................................................F.................................................. 2800/5464
[01:15:29] .................................................................................................... 3000/5464
[01:15:29] .................................................................................................... 3000/5464
[01:15:33] ...........................................................................F........................ 3100/5464
[01:15:41] .......................................................................................i............ 3300/5464
[01:15:45] .................................................................................................... 3400/5464
[01:15:49] .............................................................ii...i..ii............................. 3500/5464
[01:15:55] .................................................................................................... 3600/5464
[01:15:55] .................................................................................................... 3600/5464
[01:15:59] .................................................................................................... 3700/5464
[01:16:03] .......................................................................ii........................... 3800/5464
[01:16:05] .........................................................................................i.......... 3900/5464
[01:16:08] .................................................................................................... 4000/5464
[01:16:10] ...............................................i.................................................... 4100/5464
[01:16:13] ...................FFF..F........................................................................... 4200/5464
[01:16:31] .................................................................................................... 4400/5464
[01:16:34] .................................................................................................... 4500/5464
[01:16:38] .................................................................................................... 4600/5464
[01:16:44] .......i............................................................................................ 4700/5464
[01:16:44] .......i............................................................................................ 4700/5464
[01:16:49] .................................................................................................... 4800/5464
[01:16:53] .................................................................................................... 4900/5464
[01:16:58] .................................................................................................... 5000/5464
[01:17:01] .................................................................................................... 5100/5464
[01:17:06] .....................................F.............................................................. 5200/5464
[01:17:12] .................................................................................................... 5400/5464
[01:17:14] ..i.............................................................
[01:17:14] failures:
[01:17:14] 
[01:17:14] 
[01:17:14] ---- [ui] ui/error-codes/E0423.rs stdout ----
[01:17:14] diff of stderr:
[01:17:14] 
[01:17:14] 10    |
[01:17:14] 11 LL |     if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!("Ok"); }
[01:17:14] 12    |                                    ^
[01:17:14] - note: ...due to this, which is why a type is expected
[01:17:14] + note: ...due to this, which is why a type is expected after
[01:17:14] 14   --> $DIR/E0423.rs:12:37
[01:17:14] 15    |
[01:17:14] 16 LL |     if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!("Ok"); }
[01:17:14] 35    |
[01:17:14] 35    |
[01:17:14] 36 LL |     for _ in std::ops::Range { start: 0, end: 10 } {}
[01:17:14] 37    |                                ^^^^^
[01:17:14] - note: ...due to this, which is why a type is expected
[01:17:14] + note: ...due to this, which is why a type is expected after
[01:17:14] 39   --> $DIR/E0423.rs:21:37
[01:17:14] 40    |
[01:17:14] 41 LL |     for _ in std::ops::Range { start: 0, end: 10 } {}
[01:17:14] 
[01:17:14] The actual stderr differed from the expected stderr.
[01:17:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/E0423.stderr
[01:17:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/E0423.stderr
[01:17:14] To update references, rerun the tests and pass the `--bless` flag
[01:17:14] To only update this specific test, also pass `--test-args error-codes/E0423.rs`
[01:17:14] error: 1 errors occurred comparing output.
[01:17:14] status: exit code: 1
[01:17:14] status: exit code: 1
[01:17:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0423.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/auxiliary" "-A" "unused"
[01:17:14] ------------------------------------------
[01:17:14] 
[01:17:14] ------------------------------------------
[01:17:14] stderr:
[01:17:14] stderr:
[01:17:14] ------------------------------------------
[01:17:14] {"message":"expected type, found `1`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":203,"byte_end":204,"line_start":12,"line_end":12,"column_start":39,"column_end":40,"is_primary":true,"text":[{"text":"    if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }","highlight_start":39,"highlight_end":40}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this expression is annotated with type ascription...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":200,"byte_end":201,"line_start":12,"line_end":12,"column_start":36,"column_end":37,"is_primary":true,"text":[{"text":"    if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }","highlight_start":36,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...due to this, which is why a type is expected after","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":201,"byte_end":202,"line_start":12,"line_end":12,"column_start":37,"column_end":38,"is_primary":true,"text":[{"text":"    if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }","highlight_start":37,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this might be indicative of a syntax error elsewhere","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: expected type, found `1`\n  --> /checkout/src/test/ui/error-codes/E0423.rs:12:39\n   |\nLL |     if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }\n   |                                       ^ expecting a type here because of type ascription\n   |\n   = note: type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`\nnote: this expression is annotated with type ascription...\n  --> /checkout/src/test/ui/error-codes/E0423.rs:12:36\n   |\nLL |     if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }\n   |                                    ^\nnote: ...due to this, which is why a type is expected after\n  --> /checkout/src/test/ui/error-codes/E0423.rs:12:37\n   |\nLL |     if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!(\"Ok\"); }\n   |                                     ^\n   = help: this might be indicative of a syntax error elsewhere\n\n"}
[01:17:14] {"message":"expected expression, found `==`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":301,"byte_end":303,"line_start":15,"line_end":15,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"    if T {} == T {} { println!(\"Ok\"); }","highlight_start":13,"highlight_end":15}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `==`\n  --> /checkout/src/test/ui/error-codes/E0423.rs:15:13\n   |\nLL |     if T {} == T {} { println!(\"Ok\"); }\n   |             ^^ expected expression\n\n"}
[01:17:14] {"message":"expected type, found `0`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":449,"byte_end":450,"line_start":21,"line_end":21,"column_start":39,"column_end":40,"is_primary":true,"text":[{"text":"    for _ in std::ops::Range { start: 0, end: 10 } {}","highlight_start":39,"highlight_end":40}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this expression is annotated with type ascription...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":442,"byte_end":447,"line_start":21,"line_end":21,"column_start":32,"column_end":37,"is_primary":true,"text":[{"text":"    for _ in std::ops::Range { start: 0, end: 10 } {}","highlight_start":32,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...due to this, which is why a type is expected after","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0423.rs","byte_start":447,"byte_end":448,"line_start":21,"line_end":21,"column_start":37,"column_end":38,"is_primary":true,"text":[{"text":"    for _ in std::ops::Range { start: 0, end: 10 } {}","highlight_start":37,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this might be indicative of a syntax error elsewhere","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: expected type, found `0`\n  --> /checkout/src/test/ui/error-codes/E0423.rs:21:39\n   |\nLL |     for _ in std::ops::Range { start: 0, end: 10 } {}\n   |                                       ^ expecting a type here because of type ascription\n   |\n   = note: type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`\nnote: this expression is annotated with type ascription...\n  --> /checkout/src/test/ui/error-codes/E0423.rs:21:32\n   |\nLL |     for _ in std::ops::Range { start: 0, end: 10 } {}\n   |                                ^^^^^\nnote: ...due to this, which is why a type is expected after\n  --> /checkout/src/test/ui/error-codes/E0423.rs:21:37\n   |\nLL |     for _ in std::ops::Range { start: 0, end: 10 } {}\n   |                                     ^\n   = help: this might be indicative of a syntax error elsewhere\n\n"}
[01:17:14] {"message":"expected function, found struct `Foo`","code":{"code":"E0423","explanation":"\nAn identifier was used like a function name or a value was expected and the\nidentifier exists but it belongs to a different namespace.\n\nFor (an erroneous) example, here a `struct` variant name were used as a\nfunction:\n\n