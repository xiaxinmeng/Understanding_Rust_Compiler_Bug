plain
travis_time:end:259a08c4:start=1544367388953236259,finish=1544367446823233096,duration=57869996837
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:46:58] 
[00:46:58] running 5132 tests
[00:47:00] .................................................................................................... 100/5132
[00:47:03] ..............................................................F.F................................... 200/5132
[00:47:09] .................................................................................................... 400/5132
[00:47:12] .................................................................................................... 500/5132
[00:47:16] ..............................i..................................................................... 600/5132
[00:47:19] .................................................................................................... 700/5132
---
[00:47:42] .................................................................................................... 1400/5132
[00:47:44] .................................................................................................... 1500/5132
[00:47:47] ...........................i....................................................................i... 1600/5132
[00:47:51] .................................................................................................... 1700/5132
[00:47:54] ..........................................F......................................................... 1800/5132
[00:48:00] .....................................i.............................................................. 2000/5132
[00:48:04] .................................................................................................... 2100/5132
[00:48:08] .................................................................................................... 2200/5132
[00:48:11] .................................................................................................... 2300/5132
[00:48:11] .................................................................................................... 2300/5132
[00:48:15] ......................................................F............................................. 2400/5132
[00:48:22] .................................................................................................... 2600/5132
[00:48:27] .................................................................................................... 2700/5132
[00:48:30] .................................................................................................... 2800/5132
[00:48:33] .................................................................................................... 2900/5132
[00:48:33] .................................................................................................... 2900/5132
[00:48:37] ...................................................F................................................ 3000/5132
[00:48:40] ...F.....F........................................................................i................. 3100/5132
[00:48:47] ............................................ii..i..ii............................................... 3300/5132
[00:48:51] .................................................................................................... 3400/5132
[00:48:54] .................................................................................................... 3500/5132
[00:48:57] ............................ii...................................................................... 3600/5132
[00:48:57] ............................ii...................................................................... 3600/5132
[00:48:59] ..............................................i..................................................... 3700/5132
[00:49:00] .................................................................................................... 3800/5132
[00:49:01] ..i................................................................................................. 3900/5132
[00:49:06] .................................................................................................... 4000/5132
[00:49:11] .................................................................................................... 4100/5132
[00:49:14] .................................................................................................... 4200/5132
[00:49:18] .................................................................................................... 4300/5132
[00:49:22] ..i.........................................FF...................................................... 4400/5132
[00:49:27] ........................F........................................................................... 4500/5132
[00:49:30] ....................................................F............................................... 4600/5132
[00:49:37] .................................................................................................... 4800/5132
[00:49:40] .................................................................................................... 4900/5132
[00:49:43] .................................................................................................... 5000/5132
[00:49:43] .................................................................................................... 5000/5132
[00:49:45] .....................F.................................................i............................ 5100/5132
[00:49:46] failures:
[00:49:46] 
[00:49:46] ---- [ui] ui/bad/bad-lint-cap2.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 2   --> $DIR/bad-lint-cap2.rs:16:5
[00:49:46] 3    |
[00:49:46] 4 LL | use std::option; //~ ERROR
[00:49:46] -    |     ^^^^^^^^^^^
[00:49:46] +    | ----^^^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 7 note: lint level defined here
[00:49:46] 8   --> $DIR/bad-lint-cap2.rs:14:9
[00:49:46] 
[00:49:46] 
[00:49:46] 
[00:49:46] The actual stderr differed from the expected stderr.
[00:49:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-lint-cap2/bad-lint-cap2.stderr
[00:49:46] To update references, rerun the tests and pass the `--bless` flag
[00:49:46] To only update this specific test, also pass `--test-args bad/bad-lint-cap2.rs`
[00:49:46] error: 1 errors occurred comparing output.
[00:49:46] status: exit code: 1
[00:49:46] status: exit code: 1
[00:49:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bad/bad-lint-cap2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-lint-cap2/a" lint-cap2.rs","byte_start":540,"byte_end":556,"line_start":16,"line_end":16,"column_start":1,"column_end":17,"is_primary":true,"text":[{"text":"use std::option; //~ ERROR","highlight_start":1,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `std::option`\n  --> /checkout/src/test/ui/bad/bad-lint-cap2.rs:16:5\n   |\nLL | use std::option; //~ ERROR\n   | ----^^^^^^^^^^^- help: remove the whole `use` item\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/bad/bad-lint-cap2.rs:14:9\n   |\nLL | #![deny(warnings)]\n   |         ^^^^^^^^\n   = note: #[deny(unused_imports)] implied by #[deny(warnings)]\n\n"}
[00:49:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/bad/bad-lint-cap2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:46] 
[00:49:46] ---- [ui] ui/bad/bad-lint-cap3.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 2   --> $DIR/bad-lint-cap3.rs:17:5
[00:49:46] 3    |
[00:49:46] 4 LL | use std::option; //~ WARN
[00:49:46] -    |     ^^^^^^^^^^^
[00:49:46] +    | ----^^^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 7 note: lint level defined here
[00:49:46] 8   --> $DIR/bad-lint-cap3.rs:14:9
[00:49:46] 
[00:49:46] 
[l":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/bad/bad-lint-cap3.rs","byte_start":528,"byte_end":536,"line_start":14,"line_end":14,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"#![deny(warnings)]","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[warn(unused_imports)] implied by #[warn(warnings)]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/bad/bad-lint-cap3.rs","byte_start":571,"byte_end":587,"line_start":17,"line_end":17,"column_start":1,"column_end":17,"is_primary":true,"text":[{"text":"use std::option; //~ WARN","highlight_start":1,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::option`\n  --> /checkout/src/test/ui/bad/bad-lint-cap3.rs:17:5\n   |\nLL | use std::option; //~ WARN\n   | ----^^^^^^^^^^^- help: remove the whole `use` item\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/bad/bad-lint-cap3.rs:14:9\n   |\nLL | #![deny(warnings)]\n   |         ^^^^^^^^\n   = note: #[warn(unused_imports)] implied by #[warn(warnings)]\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/bad/bad-lint-cap3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] 
[00:49:46] 
[00:49:46] ---- [ui] ui/imports/unused.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 2   --> $DIR/unused.rs:17:24
[00:49:46] 3    |
[00:49:46] 4 LL |         pub(super) use super::f; //~ ERROR unused
[00:49:46] +    |         ---------------^^^^^^^^- help: remove the whole `use` item
[00:49:46] 6    |
[00:49:46] 7 note: lint level defined here
[00:49:46] 8   --> $DIR/unused.rs:11:9
[00:49:46] 8   --> $DIR/unused.rs:11:9
[00:49:46] 
[00:49:46] 
[00:49:46] The actual stderr differed from the expected stderr.
[00:49:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/unused/unused.stderr
[00:49:46] To update references, rerun the tests and pass the `--bless` flag
[00:49:46] To only update this specific test, also pass `--test-args imports/unused.rs`
[00:49:46] error: 1 errors occurred comparing output.
[00:49:46] status: exit code: 1
[00:49:46] status: exit code: 1
[00:49:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/unused.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/unused/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/unused/auxiliary" "-A" "unused"
[00:49:46] stdout:
[00:49:46] -----------------------------warnings)]","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(unused_imports)] implied by #[deny(warnings)]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-30730.rs","byte_start":503,"byte_end":519,"line_start":13,"line_end":13,"column_start":1,"column_end":17,"is_primary":true,"text":[{"text":"use std::thread;","highlight_start":1,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `std::thread`\n  --> /checkout/src/test/ui/issues/issue-30730.rs:13:5\n   |\nLL | use std::thread;\n   | ----^^^^^^^^^^^- help: remove the whole `use` item\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/issues/issue-30730.rs:12:9\n   |\nLL | #![deny(warnings)]\n   |         ^^^^^^^^\n   = note: #[deny(unused_imports)] implied by #[deny(warnings)]\n\n"}
[00:49:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/issues/issue-30730.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] 
[00:49:46] 
[00:49:46] ---- [ui] ui/lint/lint-directives-on-use-items-issue-10534.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 2   --> $DIR/lint-directives-on-use-items-issue-10534.rs:22:9
[00:49:46] 3    |
[00:49:46] 4 LL |     use a::x; //~ ERROR: unused import
[00:49:46] +    |     ----^^^^- help: remove the whole `use` item
[00:49:46] 6    |
[00:49:46] 7 note: lint level defined here
[00:49:46] 8   --> $DIR/lint-directives-on-use-items-issue-10534.rs:11:9
[00:49:46] 8   --> $DIR/lint-directives-on-use-items-issue-10534.rs:11:9
[00:49:46] 
[00:49:46] 14   --> $DIR/lint-directives-on-use-items-issue-10534.rs:31:9
[00:49:46] 15    |
[00:49:46] 16 LL |     use a::y; //~ ERROR: unused import
[00:49:46] +    |     ----^^^^- help: remove the whole `use` item
[00:49:46] 18    |
[00:49:46] 19 note: lint level defined here
[00:49:46] 20   --> $DIR/lint-directives-on-use-items-issue-10534.rs:30:12
[00:49:46] 20   --> $DIR/lint-directives-on-use-items-issue-10534.rs:30:12
[00:49:46] 
[00:49:46] 
[00:49:46] The actual stderr differed from the expected stderr.
[00:49:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-directives-on-use-items-issue-10534/lint-directives-on-use-items-issue-10534.stderr
[00:49:46] To update references, rerun the tests and pass the `--bless` flag
[00:49:46] To only update this specific test, also pass `--test-args lint/lint-directives-on-use-items-issue-10534.rs`
[00:49:46] error: 1 errors occurred comparing output.
[00:49:46] status: exit code: 1
[00:49:46] status: exit code: 1
[00:49:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-directives-on-use-items-issue-10534.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-p","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-directives-on-use-items-issue-10534.rs","byte_start":768,"byte_end":777,"line_start":22,"line_end":22,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    use a::x; //~ ERROR: unused import","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `a::x`\n  --> /checkout/src/test/ui/lint/lint-directives-on-use-items-issue-10534.rs:22:9\n   |\nLL |     use a::x; //~ ERROR: unused import\n   |     ----^^^^- help: remove the whole `use` item\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-directives-on-use-items-issue-10534.rs:11:9\n   |\nLL | #![deny(unused_imports)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:49:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:49:46] {"message":"unused import: `a::y`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-directives-on-use-items-issue-10534.rs","byte_start":949,"byte_end":953,"line_start":31,"line_end":31,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    use a::y; //~ ERROR: unused import","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-directives-on-use-items-issue-10534.rs",untest.rs:3252:9
[00:49:46] ---- [ui] ui/lint/lint-unused-imports.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 
[00:49:46] - error: unused import: `use std::fmt::{};`
[00:49:46] -   --> $DIR/lint-unused-imports.rs:18:1
[00:49:46] + error: unused import: `std::fmt::{}`
[00:49:46] +   --> $DIR/lint-unused-imports.rs:18:5
[00:49:46] 3    |
[00:49:46] 4 LL | use std::fmt::{};
[00:49:46] -    | ^^^^^^^^^^^^^^^^^
[00:49:46] +    | ----^^^^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 7 note: lint level defined here
[00:49:46] 8   --> $DIR/lint-unused-imports.rs:11:9
[00:49:46] 
[00:49:46] 14   --> $DIR/lint-unused-imports.rs:22:27
[00:49:46] 14   --> $DIR/lint-unused-imports.rs:22:27
[00:49:46] 15    |
[00:49:46] 16 LL | use std::option::Option::{Some, None};
[00:49:46] -    |                           ^^^^  ^^^^
[00:49:46] +    | --------------------------^^^^--^^^^-- help: remove the whole `use` item
[00:49:46] 19 error: unused import: `test::A`
[00:49:46] 20   --> $DIR/lint-unused-imports.rs:25:5
[00:49:46] 
[00:49:46] 21    |
[00:49:46] 21    |
[00:49:46] 22 LL | use test::A;       //~ ERROR unused import: `test::A`
[00:49:46] -    |     ^^^^^^^
[00:49:46] +    | ----^^^^^^^- help: remove the whole `use` item
[00:49:46] 25 error: unused import: `bar`
[00:49:46] 26   --> $DIR/lint-unused-imports.rs:34:18
[00:49:46] 
[00:49:46] 27    |
[00:49:46] 27    |
[00:49:46] 28 LL | use test2::{foo, bar}; //~ ERROR unused import: `bar`
[00:49:46] +    |                --^^^
[00:49:46] +    |                |
[00:49:46] +    |                help: remove the unused import
[00:49:46] +    |                help: remove the unused import
[00:40:49:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unused-imports.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-imports/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-imports/auxiliary" "-A" "unused"
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] ------------------------------------------
[00:49:46] stderr:
[00:49:46] stderr:
[00:49:46] ------------------------------------------
[00:49:46] {"message":"unused import: `std::fmt::{}`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":659,"byte_end":671,"line_start":18,"line_end":18,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"use std::fmt::{};","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":475,"byte_end":489,"line_start":11,"line_end":11,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"#![deny(unused_imports)]","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":655,"byte_end":672,"line_start":18,"line_end":18,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"use std::fmt::{};","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `std::fmt::{}`\n  --> /checkout/src/test/ui/lint/lint-unused-imports.rs:18:5\n   |\nLL | use std::fmt::{};\n   | ----^^^^^^^^^^^^- help: remove the whole `use` item\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-unused-imports.rs:11:9\n   |\nLL | #![deny(unused_imports)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:49:46] {"message":"unused imports: `None`, `Some`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":794,"byte_end":798,"line_start":22,"line_end":22,"column_start":27,"column_end":31,"is_primary":true,"text":[{"text":"use std::option::Option::{Some, None};","highlight_start":27,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":800,"byte_end":804,"line_start":22,"line_end":22,"column_start":33,"column_end":37,"is_primary":true,"text":[{"text":"use std::option::Option::{Some, None};","highlight_stat":2175,"byte_end":2182,"line_start":78,"line_end":78,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"    use self::g; //~ ERROR unused import: `self::g`","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":2171,"byte_end":2183,"line_start":78,"line_end":78,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    use self::g; //~ ERROR unused import: `self::g`","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `self::g`\n  --> /checkout/src/test/ui/lint/lint-unused-imports.rs:78:9\n   |\nLL |     use self::g; //~ ERROR unused import: `self::g`\n   |     ----^^^^^^^- help: remove the whole `use` item\n\n"}
[00:49:46] {"message":"unused import: `test2::foo`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":2325,"byte_end":2335,"line_start":87,"line_end":87,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"    use test2::foo; //~ ERROR unused import: `test2::foo`","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":2321,"byte_end":2336,"line_start":87,"line_end":87,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    use test2::foo; //~ ERROR unused import: `test2::foo`","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `test2::foo`\n  --> /checkout/src/test/ui/lint/lint-unused-imports.rs:87:9\n   |\nLL |     use test2::foo; //~ ERROR unused import: `test2::foo`\n   |     ----^^^^^^^^^^- help: remove the whole `use` item\n\n"}
[00:49:46] {"message":"unused import: `test::B2`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-imports.rs","byte_start":1108,"byte_end":1116,"line_start":30,"line_end":30,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use test::B2; //~ ERROR unused import: `test::B2`","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unused import: `test::B2`\n  --> /checkout/src/test/ui/lint/lint-unused-imports.rs:30:5\n   |\nLL | use test::B2; //~ ERROR unused import: `test::B2`\n   |     ^^^^^^^^\n\n"}
[00:49:46] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/lint/lint-unused-imports.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] 
[00:49:46] 
[00:49:46] ---- [ui] ui/lint/lints-in-foreign-macros.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 2   --> $DIR/lints-in-foreign-macros.rs:21:16
[00:49:46] 3    |
[00:49:46] 4 LL |     () => {use std::string::ToString;} //~ WARN: unused import
[00:49:46] +    |            ----^^^^^^^^^^^^^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 6 ...
[00:49:46] 6 ...
[00:49:46] 7 LL | mod a { foo!(); }
[00:49:46] 
[00:49:46] 17   --> $DIR/lints-in-foreign-macros.rs:26:18
[00:49:46] 18    |
[00:49:46] 18    |
[00:49:46] 19 LL | mod c { baz!(use std::string::ToString;); } //~ WARN: unused import
[00:49:46] +    |              ----^^^^^^^^^^^^^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 21 
[00:49:46] 22 warning: unused import: `std::string::ToString`
[00:49:46] 23   --> $DIR/lints-in-foreign-macros.rs:27:19
[00:49:46] 23   --> $DIR/lints-in-foreign-macros.rs:27:19
[00:49:46] 
[00:49:46] 24    |
[00:49:46] 25 LL | mod d { baz2!(use std::string::ToString;); } //~ WARN: unused import
[00:49:46] +    |               ----^^^^^^^^^^^^^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 27 
[00:49:46] 28 warning: missing documentation for crate
[00:49:46] 29   --> $DIR/lints-in-foreign-macros.rs:14:1
[00:49:46] 29   --> $DIR/lints-in-foreign-macros.rs:14:1
[00:49:46] 
[00:49:46] 
[00:49:46] The actual stderr differed from the expected stderr.
[00:49:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/lints-in-foreign-macros.stderr
[00:49:46] To update references, rerun the tests and pass the `--bless` flag
[00:49:46] To only update this specific test, also pass `--test-args lint/lints-in-foreign-macros.rs`
[00:49:46] error: 1 errors occurred comparing output.
[00:49:46] status: exit code: 0
[00:49:46] status: exit code: 0
[00:49:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lints-in-foreign-macros.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/auxiliary" "-A" "unused"
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] ------------------------------------------
[00:49:46] stderr:
[00:49:46] stderr:
[00:49:46] ------------------------------------------
[00:49:46] {"message":"unused import: `std::string::ToString`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":710,"byte_end":731,"line_start":21,"line_end":21,"column_start":16,"column_end":37,"is_primary":true,"text":[{"text":"    () => {use std::string::ToString;} //~ WARN: unused import","highlight_start":16,"highlight_end":37lints-in-foreign-macros.rs","byte_start":706,"byte_end":732,"line_start":21,"line_end":21,"column_start":12,"column_end":38,"is_primary":true,"text":[{"text":"    () => {use std::string::ToString;} //~ WARN: unused import","highlight_start":12,"highlight_end":38}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":769,"byte_end":776,"line_start":24,"line_end":24,"column_start":9,"column_end":16,"is_primary":false,"text":[{"text":"mod a { foo!(); }","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"foo!","def_site_span":{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":676,"byte_end":759,"line_start":20,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! foo {","highlight_start":1,"highlight_end":19},{"text":"    () => {use std::string::ToString;} //~ WARN: unused import","highlight_start":1,"highlight_end":63},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::string::ToString`\n  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:21:16\n   |\nLL |     () => {use std::string::ToString;} //~ WARN: unused import\n   |            ----^^^^^^^^^^^^^^^^^^^^^- help: remove the whole `use` item\n...\nLL | mod a { foo!(); }\n   |         ------- in this macro invocation\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:14:9\n   |\nLL | #![warn(unused_imports)] //~ missing documentation for crate [missing_docs]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:49:46] {"message":"unused import: `std::string::ToString`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":814,"byte_end":835,"line_start":26,"line_end":26,"column_start":18,"column_end":39,"is_primary":true,"text":[{"text":"mod c { baz!(use std::string::ToString;); } //~ WARN: unused import","highlight_start":18,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":810,"byte_end":836,"line_start":26,"line_end":26,"column_start":14,"column_end":40,"is_primary":true,"text":[{"text":"mod c { baz!(use std::string::ToString;); } //~ WARN: unused import","highlight_start":14,"highlight_end":40}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::string::ToString`\n  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:26:18\n   |\nLL | mod c { baz!(use std::string::ToString;); } //~ WARN: unused import\n   |              ----^^^^^^^^^^^^^^^^^^^^^- help: remove the whole `use` item\n\n"}
[00:49:46] {"message":"unuseds_primary":true,"text":[{"text":"#![warn(unused_imports)] //~ missing documentation for crate [missing_docs]","highlight_start":1,"highlight_end":76},{"text":"#![warn(missing_docs)]","highlight_start":1,"highlight_end":23},{"text":"","highlight_start":1,"highlight_end":1},{"text":"#[macro_use]","highlight_start":1,"highlight_end":13},{"text":"extern crate lints_in_foreign_macros;","highlight_start":1,"highlight_end":38},{"text":"","highlight_start":1,"highlight_end":1},{"text":"macro_rules! foo {","highlight_start":1,"highlight_end":19},{"text":"    () => {use std::string::ToString;} //~ WARN: unused import","highlight_start":1,"highlight_end":63},{"text":"}","highlight_start":1,"highlight_end":2},{"text":"","highlight_start":1,"highlight_end":1},{"text":"mod a { foo!(); }","highlight_start":1,"highlight_end":18},{"text":"mod b { bar!(); }","highlight_start":1,"highlight_end":18},{"text":"mod c { baz!(use std::string::ToString;); } //~ WARN: unused import","highlight_start":1,"highlight_end":68},{"text":"mod d { baz2!(use std::string::ToString;); } //~ WARN: unused import","highlight_start":1,"highlight_end":69},{"text":"baz!(pub fn undocumented() {}); //~ WARN: missing documentation for a function","highlight_start":1,"highlight_end":79},{"text":"baz2!(pub fn undocumented2() {}); //~ WARN: missing documentation for a function","highlight_start":1,"highlight_end":81},{"text":"","highlight_start":1,"highlight_end":1},{"text":"fn main() {}","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code": WARN: missing documentation for a function\n   |      ^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:46] {"message":"missing documentation for a function","code":{"code":"missing_docs","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":1019,"byte_end":1041,"line_start":29,"line_end":29,"column_start":7,"column_end":29,"is_primary":true,"text":[{"text":"baz2!(pub fn undocumented2() {}); //~ WARN: missing documentation for a function","highlight_start":7,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: missing documentation for a function\n  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:29:7\n   |\nLL | baz2!(pub fn undocumented2() {}); //~ WARN: missing documentation for a function\n   |       ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/lint/lints-in-foreign-macros.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] 
[00:49:46] 
[00:49:46] ---- [ui] ui/rfc-2166-underscore-imports/unused-2018.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 2   --> $DIR/unused-2018.rs:7:9
[00:49:46] 3    |
[00:49:46] 4 LL |     use core::any; //~ ERROR unused import: `core::any`
[00:49:46] +    |     ----^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 6    |
[00:49:46] 7 note: lint level defined here
[00:49:46] 8   --> $DIR/unused-2018.rs:4:9
[00:49:46] 8   --> $DIR/unused-2018.rs:4:9
[00:49:46] 
[00:49:46] 14   --> $DIR[00:49:46] {"message":"unused import: `core::any`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs","byte_start":103,"byte_end":112,"line_start":7,"line_end":7,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"    use core::any; //~ ERROR unused import: `core::any`","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs","byte_start":57,"byte_end":71,"line_start":4,"line_end":4,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"#![deny(unused_imports)]","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs","byte_start":99,"byte_end":113,"line_start":7,"line_end":7,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    use core::any; //~ ERROR unused import: `core::any`","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `core::any`\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs:7:9\n   |\nLL |     use core::any; //~ ERROR unused import: `core::any`\n   |     ----^^^^^^^^^- help: remove the whole `use` item\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs:4:9\n   |\nLL | #![deny(unused_imports)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:49:46] {"message":"unused import: `core`","code":{"code":"unused_imports","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs","byte_start":183,"byte_end":187,"line_start":11,"line_end":11,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    use core; //~ ERROR unused import: `core`","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the whole `use` item","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs","byte_start":179,"byte_end":188,"line_start":11,"line_end":11,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    use core; //~ ERROR unused import: `core`","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused import: `core`\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/unused-2018.rs:11:9\n   |\nLL |     use core; //~ ERROR unused import: `core`\n   |     ----^^^^- help: remove the whole `use` item\n\n"}
[00:49:46] {"message":"aborting due to 2 previous errors","code":ni/rfc-2166-underscore-imports/basic.rs","byte_start":931,"byte_end":942,"line_start":38,"line_end":38,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    use S as _; //~ WARN unused import","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `S as _`\n  --> /checkout/src/test/ui/rfc-2166-underscore-imports/basic.rs:38:9\n   |\nLL |     use S as _; //~ WARN unused import\n   |     ----^^^^^^- help: remove the whole `use` item\n\n"}
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] thread '[ui] ui/rfc-2166-underscore-imports/basic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:46] 
[00:49:46] 
[00:49:46] ---- [ui] ui/rust-2018/uniform-paths/issue-54390.rs stdout ----
[00:49:46] diff of stderr:
[00:49:46] 
[00:49:46] 18   --> $DIR/issue-54390.rs:8:5
[00:49:46] 19    |
[00:49:46] 20 LL | use fmt::Write; //~ ERROR imports can only refer to extern crate names
[00:49:46] -    |     ^^^^^^^^^^
[00:49:46] +    | ----^^^^^^^^^^- help: remove the whole `use` item
[00:49:46] 23 note: lint level defined here
[00:49:46] 24   --> $DIR/issue-54390.rs:3:9
[00:49:46] 
[00:49:46] 
[00:49:46] 
[00:49:46] The actual stderr differed from the expected stderr.
[00:49:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-54390/issue-54390.stderr
[00:49:46] To update references, rerun the tests and pass the `--bless` flag
[00:49:46] To only update this specific test, also pass `--test-args rust-2018/uniform-paths/issue-54390.rs`
[00:49:46] error: 1 errors occurred comparing output.
[00:49:46] status: exit code: 1
[00:49:46] status: exit code: 1
[00:49:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-54390/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-54390/auxiliary" "-A" "unused"
[00:49:46] ------------------------------------------
[00:49:46] 
[00:49:46] ------------------------------------------
[00:49:46] stderr:
[00:49:46] stderr:
[00:49:46] ------------------------------------------
[00:49:46] {"message":"imports can only refer to extern crate names passed with `--extern` on stable channel (see issue #53130)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n