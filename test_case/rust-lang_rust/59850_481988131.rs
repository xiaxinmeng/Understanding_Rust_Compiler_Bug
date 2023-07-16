plain
travis_time:end:1376497c:start=1554958250268005785,finish=1554958252511574053,duration=2243568268
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:40] .................................................................................................... 3000/5530
[01:09:45] .................................................................................................... 3100/5530
[01:09:48] .................................................................................................... 3200/5530
[01:09:53] .................................................................................................... 3300/5530
[01:09:57] ..............................i................................................................F.... 3400/5530
[01:10:05] ....ii...i..ii...................................................................................... 3600/5530
[01:10:10] .................................................................................................... 3700/5530
[01:10:15] .................................................................................................... 3800/5530
[01:10:19] ...............ii................................................................................... 3900/5530
---
[01:10:44] .................................................................................................... 4400/5530
[01:10:48] .................................................................................................... 4500/5530
[01:10:52] .................................................................................................... 4600/5530
[01:10:56] .................................................................................................... 4700/5530
[01:11:04] ...............................................................................F.................... 4800/5530
[01:11:11] .................................................................................................... 5000/5530
[01:11:17] .................................................................................................... 5100/5530
[01:11:20] .................................................................................................... 5200/5530
[01:11:25] .................................................................................................... 5300/5530
---
[01:11:32] diff of stderr:
[01:11:32] 
[01:11:32] 27    = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
[01:11:32] 28 
[01:11:32] 29 warning: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...",
[01:11:32] -                                              /*opt*/ cfg = "...")]`
[01:11:32] +                                             /*opt*/ cfg = "...")]`
[01:11:32] 32    |
[01:11:32] 32    |
[01:11:32] 33 LL | #[link]
[01:11:32] 37    = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>
[01:11:32] 38 
[01:11:32] 38 
[01:11:32] 39 warning: attribute must be of the form `#[link(name = "...", /*opt*/ kind = "dylib|static|...",
[01:11:32] -                                              /*opt*/ cfg = "...")]`
[01:11:32] +                                             /*opt*/ cfg = "...")]`
[01:11:32] 42    |
[01:11:32] 42    |
[01:11:32] 43 LL | #[link = ""]
[01:11:32] 
[01:11:32] The actual stderr differed from the expected stderr.
[01:11:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/malformed-regressions.stderr
[01:11:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/malformed-regressions.stderr
[01:11:32] To update references, rerun the tests and pass the `--bless` flag
[01:11:32] To only update this specific test, also pass `--test-args malformed/malformed-regressions.rs`
[01:11:32] error: 1 errors occurred comparing output.
[01:11:32] status: exit code: 0
[01:11:32] status: exit code: 0
[01:11:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/malformed/malformed-regressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/auxiliary" "-A" "unused"
[01:11:32] ------------------------------------------
[01:11:32] 
[01:11:32] ------------------------------------------
[01:11:32] stderr:
[01:11:32] stderr:
[01:11:32] ------------------------------------------
[01:11:32] {"message":"attribute must be of the form `#[doc(hidden|inline|...)]` or `#[doc = \"string\"]`","code":{"code":"ill_formed_attribute_input","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/malformed/malformed-regressions.rs","byte_start":17,"byte_end":23,"line_start":3,"line_end":3,"column_start":1,"column_end":7,"is_primary":true,"text":[{"text":"#[doc] //~ WARN attribute must be of the form","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(ill_formed_attribute_input)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attribute must be of the form `#[doc(hidden|inline|...)]` or `#[doc = \"string\"]`\n  --> /checkout/src/test/ui/malformed/malformed-regressions.rs:3:1\n   |\nLL | #[doc] //~ WARN attribute must be of the form\n   | ^^^^^^\n   |\n   = note: #[warn(ill_formed_attribute_input)] on by default\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>\n\n"}
[01:11:32] {"message":"attribute must be of the form `#[ignore]` or `#[ignore = \"reason\"]`","code":{"code":"ill_formed_attribute_input","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/malformed/malformed-regressions.rs","byte_start":63,"byte_end":74,"line_start":4,"line_end":4,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"#[ignore()] //~ WARN attribute must be of the form","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attribute must be of the form `#[ignore]` or `#[ignore = \"reason\"]`\n  --> /checkout/src/test/ui/malformed/malformed-regressions.rs:4:1\n   |\nLL | #[ignore()] //~ WARN attribute must be of the form\n   | ^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>\n\n"}
[01:11:32] {"message":"attribute must be of the form `#[inline]` or `#[inline(always|never)]`","code":{"code":"ill_formed_attribute_input","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/malformed/malformed-regressions.rs","byte_start":114,"byte_end":128,"line_start":5,"line_end":5,"column_start":1,"column_end":15,"is_primary":true,"text":[{"text":"#[inline = \"\"] //~ WARN attribute must be of the form","highlight_start":1,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attribute must be of the form `#[inline]` or `#[inline(always|never)]`\n  --> /checkout/src/test/ui/malformed/malformed-regressions.rs:5:1\n   |\nLL | #[inline = \"\"] //~ WARN attribute must be of the form\n   | ^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>\n\n"}
[01:11:32] {"message":"attribute must be of the form `#[link(name = \"...\", /*opt*/ kind = \"dylib|static|...\",\n                                            /*opt*/ cfg = \"...\")]`","code":{"code":"ill_formed_attribute_input","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/malformed/malformed-regressions.rs","byte_start":168,"byte_end":175,"line_start":6,"line_end":6,"column_start":1,"column_end":8,"is_primary":true,"text":[{"text":"#[link] //~ WARN attribute must be of the form","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attribute must be of the form `#[link(name = \"...\", /*opt*/ kind = \"dylib|static|...\",\n                                            /*opt*/ cfg = \"...\")]`\n  --> /checkout/src/test/ui/malformed/malformed-regressions.rs:6:1\n   |\nLL | #[link] //~ WARN attribute must be of the form\n   | ^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>\n\n"}
[01:11:32] {"message":"attribute must be of the form `#[link(name = \"...\", /*opt*/ kind = \"dylib|static|...\",\n                                            /*opt*/ cfg = \"...\")]`","code":{"code":"ill_formed_attribute_input","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/malformed/malformed-regressions.rs","byte_start":215,"byte_end":227,"line_start":7,"line_end":7,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"#[link = \"\"] //~ WARN attribute must be of the form","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attribute must be of the form `#[link(name = \"...\", /*opt*/ kind = \"dylib|static|...\",\n                                            /*opt*/ cfg = \"...\")]`\n  --> /checkout/src/test/ui/malformed/malformed-regressions.rs:7:1\n   |\nLL | #[link = \"\"] //~ WARN attribute must be of the form\n   | ^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #57571 <https://github.com/rust-lang/rust/issues/57571>\n\n"}
[01:11:32] ------------------------------------------
[01:11:32] 
[01:11:32] thread '[ui] ui/malformed/malformed-regressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:11:32] 
[01:11:32] 
[01:11:32] ---- [ui] ui/rust-2018/uniform-paths/macro-rules.rs stdout ----
[01:11:32] 
[01:11:32] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:32] status: exit code: 101
[01:11:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/macro-rules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/macro-rules/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/macro-rules/auxiliary" "-A" "unused"
[01:11:32] ------------------------------------------
[01:11:32] 
[01:11:32] ------------------------------------------
[01:11:32] stderr:
[01:11:32] stderr:
[01:11:32] ------------------------------------------
[01:11:32] {"message":"`legacy_macro` is private, and cannot be re-exported","code":{"code":"E0364","explanation":"\nPrivate items cannot be publicly re-exported. This error indicates that you\nattempted to `pub use` a type or value that was not itself public.\n\nErroneous code example:\n\n