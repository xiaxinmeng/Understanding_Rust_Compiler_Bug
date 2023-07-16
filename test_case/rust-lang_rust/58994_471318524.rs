plain
travis_time:end:0278f21a:start=1552229159525061602,finish=1552229230684904246,duration=71159842644
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:08:03] .................................................................................................... 300/5446
[01:08:06] .................................................................................................... 400/5446
[01:08:09] .................................................................................................... 500/5446
[01:08:13] ....................................i............................................................... 600/5446
[01:08:16] ...............................................................................................F.... 700/5446
[01:08:25] ............................................................................................i....... 900/5446
[01:08:25] ............................................................................................i....... 900/5446
[01:08:29] ........i....................F........F............................................................. 1000/5446
[01:08:32] .......................iiiii........................................................................ 1100/5446
[01:08:38] .................................................................................................... 1300/5446
[01:08:40] .................................................................................................... 1400/5446
[01:08:43] .................................................................................................... 1500/5446
[01:08:46] .................................................................................................... 1600/5446
---
[01:11:05] 
[01:11:05] ---- [ui] ui/conditional-compilation/cfg-attr-multi-true.rs stdout ----
[01:11:05] diff of stderr:
[01:11:05] 
[01:11:05] - warning: use of deprecated item 'MustUseDeprecated'
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL | impl MustUseDeprecated { //~ warning: use of deprecated item
[01:11:05] -    |
[01:11:05] -    = note: #[warn(deprecated)] on by default
[01:11:05] - 
[01:11:05] - 
[01:11:05] - warning: use of deprecated item 'MustUseDeprecated'
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |     MustUseDeprecated::new(); //~ warning: use of deprecated item
[01:11:05] - 
[01:11:05] - 
[01:11:05] - warning: use of deprecated item 'MustUseDeprecated'
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |     fn new() -> MustUseDeprecated { //~ warning: use of deprecated item
[01:11:05] -    |                 ^^^^^^^^^^^^^^^^^
[01:11:0Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/auxiliary" "-A" "unused"
[01:11:05] ------------------------------------------
[01:11:05] 
[01:11:05] ------------------------------------------
[01:11:05] stderr:
[01:11:05] stderr:
[01:11:05] ------------------------------------------
[01:11:05] {"message":"unused `MustUseDeprecated` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":524,"byte_end":549,"line_start":19,"line_end":19,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    MustUseDeprecated::new(); //~ warning: use of deprecated item","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":212,"byte_end":227,"line_start":7,"line_end":7,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused `MustUseDeprecated` that must be used\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:19:5\n   |\nLL |     MustUseDeprecated::new(); //~ warning: use of deprecated item\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:7:9\n   |\nLL | #![warn(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}
[01:11:05] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":524,"byte_end":546,"line_start":19,"line_end":19,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    MustUseDeprecated::new(); //~ warning: use of deprecated item","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(deprecated)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:19:5\n   |\nLL |     MustUseDeprecated::new(); //~ warning: use of deprecated item\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(deprecated)] on by default\n\n"}
[01:11:05] ------------------------------------------
[01:11:05] 
[01:11:05] thread '[ui] ui/conditional-compilation/cfg-attr-multi-true.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:05] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:05] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:05] 
[01:11:05] ---- [ui] ui/deprecation/deprecation-lint-nested.rs stdout ----
[01:11:05] diff of stderr:
[01:11:05] 
[01:11:05] 16 LL |     impl DeprecatedTrait for Foo {} //~ ERROR use of deprecated item
[01:11:05] 18 
[01:11:05] 18 
[01:11:05] - error: use of deprecated item 'loud::DEPRECATED_STATIC'
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         DEPRECATED_STATIC + //~ ERROR use of deprecated item
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'loud::DEPRECATED_CONST'
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         DEPRECATED_CONST //~ ERROR use of deprecated item
[01:11:05] - 
[01:11:05] - 
[01:11:05] 31 error: use of deprecated item 'loud::DeprecatedTrait'
[01:11:05] 33    |
[01:11:05] 
[01:11:05] 39    |
[01:11:05] 39    |
[01:11:05] 40 LL |             deprecated_fn(); //~ ERROR use of deprecated item
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'loud::DEPRECATED_STATIC'
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         DEPRECATED_STATIC + //~ ERROR use of deprecated item
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'loud::DEPRECATED_CONST'
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         DEPRECATED_CONST //~ ERROR use of deprecated item
[01:11:05] 42 
[01:11:05] 43 error: aborting due to 6 previous errors
[01:11:05] 44 
[01:11:05] 
[01:11:05] 
[01:11:05] 
[01:11:05] The actual stderr differed from the expected stderr.
[01:11:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecation-lint-nested/deprecation-lint-nested.stderr
[01:11:05] To update references, rerun the tests and pass the `--bless` flag
[01:11:05] To only update this specific test, also pass `--test-args deprecation/deprecation-lint-nested.rs`
[01:11:05] error: 1 errors occurred comparing output.
[01:11:05] status: exit code: 1
[01:11:05] status: exit code: 1
[01:11:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deprecation/deprecation-lint-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecation-lint-nested/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecation-lint-nested/auxiliary" "-A" "unused"
[01:11:05] ------------------------------------------
[01:11:05] 
[01:11:05] ------------------------------------------
[01:11:05] stderr:
[01:11:05] stderr:
[01:11:05] ------------------------------------------
[01:11:05] {"message":"use of deprecated item 'loud::DeprecatedType'","code":{"code":"deprecated","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/deprecation/deprecation-lint-nested.rs","byte_start":1008,"byte_end":1022,"line_start":55,"line_end":55,"column_start":16,"column_end":30,"is_primary":true,"text":[{"text":"    struct Foo(DeprecatedType); //~ ERROR use of deprecated item","highlight_start":16,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/deprecation/deprecation-lint-nested.rs","byte_start":8,"byte_end":18,"line_start":1,"line_end":1,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"#![deny(deprecated)]","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of deprecated item 'loud::DeprecatedType'\n  --> /checkout/src/test/ui/deprecation/deprecation-lint-nested.rs:55:16\n   |\nLL |     struct Foo(DeprecatedType); //~ ERROR use of deprecated item\n   |                ^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/deprecation/deprecation-lint-nested.rs:1:9\n   |\nLL | #![deny(deprecated)]\n   |         ^^^^^^^^^^\n\n"}
[01:11:05] {"message":"use of deprecated item 'loud::DeprecatedTrait'","code":{"code":"deprecated","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/deprecation/deprecation-lint-nested.rs","byte_start":1068,"byte_end":1083,"line_start":57,"line_end":57,"column_start":10,"column_end":25,"is_primary":true,"text":[{"text":"    impl DeprecatedTrait for Foo {} //~ ERROR use of deprecated item","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of deprecated item 'loud::DeprecatedTrait'\n  --> /checkout/src/test/ui/deprecation/deprecation-lint-nested.rs:57:10\n   |\nLL |     impl DeprecatedTrait for Foo {} //~ ERROR use of deprecated item\n   |          ^^^^^^^^^^^^^^^\n\n"}
[01:11:05] {"message":"use of deprecated item 'loud::DeprecatedTrait'","code":{"code":"deprecated","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/deprecation/deprecation-lint-nested.rs","byte_start":1162,"byte_end":1177,"line_start":60,"line_end":60,"column_start":19,"column_end":34,"is_primary":true,"text":[{"text":"        fn bar<T: DeprecatedTrait>() { //~ ERROR use of deprecated item","highlight_start":19,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of deprecated item 'loud::DeprecatedTrait'\n  --> /checkout/src/test/ui/deprecation/deprecation-lint-nested.rs:60:19\n   |\nLL |         fn bar<T: DeprecatedTrait>() { //~ ERROR use of deprecated item\n   |                   ^^^^^^^^^^^^^^^\n\n"}
[01:11:05] {"message":"use of deprecated item 'loud::deprecated_fn'","code":{"code":"deprecated","explanation":null},"lev:[{"file_name":"/checkout/src/test/ui/deprecation/deprecation-lint-nested.rs","byte_start":1385,"byte_end":1401,"line_start":67,"line_end":67,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        DEPRECATED_CONST //~ ERROR use of deprecated item","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of deprecated item 'loud::DEPRECATED_CONST'\n  --> /checkout/src/test/ui/deprecation/deprecation-lint-nested.rs:67:9\n   |\nLL |         DEPRECATED_CONST //~ ERROR use of deprecated item\n   |         ^^^^^^^^^^^^^^^^\n\n"}
[01:11:05] 
[01:11:05] ------------------------------------------
[01:11:05] 
[01:11:05] thread '[ui] ui/deprecation/deprecation-lint-nested.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:05] thread '[ui] ui/deprecation/deprecation-lint-nested.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:11:05] 
[01:11:05] ---- [ui] ui/deprecation/deprecation-lint.rs stdout ----
[01:11:05] diff of stderr:
[01:11:05] 
[01:11:05] 10 LL | #![deny(deprecated)]
[01:11:05] 11    |         ^^^^^^^^^^
[01:11:05] 12 
[01:11:05] + error: use of deprecated item 'deprecation_lint::MethodTester::method_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.method_deprecated(); //~ ERROR use of deprecated item 'deprecation_lint::MethodTester::method_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] 13 error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] 15    |
[01:11:05] 15    |
[01:11:05] 16 LL |         Trait::trait_deprecated(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] 
[01:11:05] 28 LL |         deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::deprecated_text': text
[01:11:05] 30 
[01:11:05] 30 
[01:11:05] + error: use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.method_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] 31 error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] 33    |
[01:11:05] 33    |
[01:11:05] 34 LL |         Trait::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] 
[01:11:05] 40 LL |         <Foo as Trait>::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] 42 
[01:11:05] 42 
[01:11:05] + error: use of deprecated item 'deprecation_lint::DeprecatedStruct::i': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |             i: 0 //~ ERROR use of deprecated item 'deprecation_lint::DeprecatedStruct::i': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] 43 error: use of deprecated item 'deprecation_lint::DeprecatedStruct': text
[01:11:05] 45    |
[01:11:05] 
[01:11:05] 
[01:11:05] 64 LL |         let _ = DeprecatedTupleStruct (1); //~ ERROR use of deprecated item 'deprecation_lint::DeprecatedTupleStruct': text
[01:11:05] 66 
[01:11:05] 66 
[01:11:05] + error: use of deprecated item 'deprecation_lint::nested::DeprecatedStruct::i': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |             i: 0 //~ ERROR use of deprecated item 'deprecation_lint::nested::DeprecatedStruct::i': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] 67 error: use of deprecated item 'deprecation_lint::nested::DeprecatedStruct': text
[01:11:05] 69    |
[01:11:05] 
[01:11:05] 101    |                                         ^^^^^^^^^^^^^^^
[01:11:05] 102 
[01:11:05] 102 
[01:11:05] 103 error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] 105    |
[01:11:05] 105    |
[01:11:05] 106 LL |         Trait::trait_deprecated(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] 113    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:11:05] 114 
[01:11:05] 114 
[01:11:05] 115 error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] 117    |
[01:11:05] 117    |
[01:11:05] 118 LL |         Trait::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] 
[01:11:05] 124 LL |         <Foo as Trait>::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] 126 
[01:11:05] 126 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] 127 error: use of deprecated item 'deprecation_lint::DeprecatedTrait': text
[01:11:05] 129    |
[01:11:05] 
[01:11:05] 
[01:11:05] 136 LL |     trait LocalTrait : DeprecatedTrait { } //~ ERROR use of deprecated item 'deprecation_lint::DeprecatedTrait': text
[01:11:05] 138 
[01:11:05] 138 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Stable::override2': text
[01:11:05] +    |
[01:11:05] + LL |             override2: 3,
[01:11:05] +    |             ^^^^^^^^^^^^
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Stable::override2': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         let _ = x.override2;
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Stable::override2': text
[01:11:05] +    |
[01:11:05] + LL |             override2: _
[01:11:05] +    |             ^^^^^^^^^^^^
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Stable2::2': text
[01:11:05] +    |
[01:11:05] + LL |         let _ = x.2;
[01:11:05] +    |                   ^
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Stable2::2': text
[01:11:05] +    |
[01:11:05] + LL |                    _)
[01:11:05] +    |                    ^
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Deprecated::inherit': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |             inherit: 1,
[01:11:05] + 
[01:11:05] + 
[01:11:05] 139 error: use of deprecated item 'deprecation_lint::Deprecated': text
[01:11:05] 141    |
[01:11:05] 
[01:11:05] 142 LL |         let x = Deprecated {
[01:11:05] 143    |                 ^^^^^^^^^^
[01:11:05] 143    |                 ^^^^^^^^^^
[01:11:05] 144 
[01:11:05] + error: use of deprecated item int.rs:145:14
[01:11:05] +    |
[01:11:05] + LL |             (_,
[01:11:05] +    |              ^
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Deprecated2::1': text
[01:11:05] +    |
[01:11:05] + LL |              _,
[01:11:05] +    |              ^
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'deprecation_lint::Deprecated2::2': text
[01:11:05] +    |
[01:11:05] + LL |              _)
[01:11:05] +    |              ^
[01:11:05] + 
[01:11:05] + 
[01:11:05] 163 error: use of deprecated item 'deprecation_lint::Deprecated2': text
[01:11:05] 165    |
[01:11:05] 
[01:11:05] 
[01:11:05] 184 LL |         deprecated(); //~ ERROR use of deprecated item 'this_crate::deprecated'
[01:11:05] 186 
[01:11:05] 186 
[01:11:05] + error: use of deprecated item 'this_crate::MethodTester::method_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.method_deprecated(); //~ ERROR use of deprecated item 'this_crate::MethodTester::method_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] 187 error: use of deprecated item 'this_crate::Trait::trait_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] + erro error: use of deprecated item 'this_crate::Trait::trait_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'this_crate::Trait::trait_deprecated': text
[01:11:05] 279    |
[01:11:05] 279    |
[01:11:05] 280 LL |         Trait::trait_deprecated(&foo); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated'
[01:11:05] 287    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:11:05] 288 
[01:11:05] 288 
[01:11:05] 289 error: use of deprecated item 'this_crate::Trait::trait_deprecated_text': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated_text': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'this_crate::Trait::trait_deprecated_text': text
[01:11:05] 291    |
[01:11:05] 291    |
[01:11:05] 292 LL |         Trait::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated_text': text
[01:11:05] 
[01:11:05] 298 LL |         <Foo as Trait>::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated_text': text
[01:11:05] 300 
[01:11:05] 300 
[01:11:05] + error: use of deprecated item 'this_crate::Trait::trait_deprecated': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated'
[01:11:05] + 
[01:11:05] + 
[01:11:05] + error: use of deprecated item 'this_crate::Trait::trait_deprecated_text': text
[01:11:05] +    |
[01:11:05] +    |
[01:11:05] + LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'this_crate::Trait::trait_deprecated_text': text
[01:11:05] + 
[01:11:05] + 
[01:11:05] 301 error: use of deprecated item 'this_crate::test_fn_closure_body::{{closure}}::bar'
[01:11:05] 303    |
[01:11:05] 
[01:11:05] 
[01:11:05] 316 LL |     trait LocalTrait : DeprecatedTrait { } //~ ERROR use of deprecated item 'this_crate::DeprecatedTrait': text
[01:11:05] 318 
[01:11:05] 318 
[01:11:05] - error: use of deprecated item 'this_crate2::Deprecated': text
[01:11:05] -    |
[01:11:05] - LL |         let x = Deprecated {
[01:11:05] -    |                 ^^^^^^^^^^
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'this_crate2::Deprecated': text
[01:11:05] -    |
[01:11:05] - LL |         let Deprecated {
[01:11:05] -    |             ^^^^^^^^^^
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecatecated'
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::MethodTester::method_deprecated': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         <Foo>::method_deprecated(&foo); //~ ERROR use of deprecated item 'deprecation_lint::MethodTester::method_deprecated'
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         <Foo>::trait_deprecated(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         foo.method_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         Foo::method_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         <Foo>::method_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::MethodTester::method_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         <Foo>::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::DeprecatedStruct::i': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |             i: 0 //~ ERROR use of deprecated item 'deprecation_lint::DeprecatedStruct::i': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::nested::DeprecatedStruct::i': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |             i: 0 //~ ERROR use of deprecated item 'deprecation_lint::nested::DeprecatedStruct::i': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         <Foo>::trait_deprecated(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         <Foo>::trait_deprecated_text(&foo); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         foo.trait_deprecated(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated'
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         foo.trait_deprecated_text(); //~ ERROR use of deprecated item 'deprecation_lint::Trait::trait_deprecated_text': text
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Stable::override2': text
[01:11:05] -    |
[01:11:05] - LL |             override2: 3,
[01:11:05] -    |             ^^^^^^^^^^^^
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Stable::override2': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         let _ = x.override2;
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Stable::override2': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |             override2: _
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Stable2::2': text
[01:11:05] -    |
[01:11:05] - LL |         let _ = x.2;
[01:11:05] -    |                 ^^^
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Stable2::2': text
[01:11:05] -    |
[01:11:05] - LL |                    _)
[01:11:05] -    |                    ^
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Deprecated::inherit': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |             inherit: 1,
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Deprecated::inherit': text
[01:11:05] -    |
[01:11:05] -    |
[01:11:05] - LL |         let _ = x.inherit;
[01:11:05] - 
[01:11:05] - 
[01:11:05] - error: use of deprecated item 'deprecation_lint::Deprecated::inherit': text
---
[01:11:05] test result: FAILED. 5421 passed; 3 failed; 22 ignored; 0 measured; 0 filtered out
[01:11:05] 
[01:11:05] 
[01:11:05] 
[01:11:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:05] 
[01:11:05] 
[01:11:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:05] Build completed unsuccessfully in 0:04:07
[01:11:05] Build completed unsuccessfully in 0:04:07
[01:11:05] make: *** [check] Error 1
[01:11:05] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04704f48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 10 15:58:25 UTC 2019
---
travis_time:end:1c97825a:start=1552233506951807513,finish=1552233506958365184,duration=6557671
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01725c2a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1881bee8
$ dmesg | grep -i kill
