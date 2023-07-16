plain
travis_time:end:01822929:start=1544291897429642780,finish=1544291974559362427,duration=77129719647
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:45:27] .................................................................................................... 500/5131
[00:45:30] ..............................i..................................................................... 600/5131
[00:45:34] .................................................................................................... 700/5131
[00:45:39] .................................................................................................... 800/5131
[00:45:43] .i............F..i.................................................................................. 900/5131
[00:45:47] ........................iiiii....................................................................... 1000/5131
[00:45:51] .................................................................................................... 1200/5131
[00:45:54] .................................................................................................... 1300/5131
[00:45:54] .................................................................................................... 1300/5131
[00:45:56] ..........F......................................................................................... 1400/5131
[00:46:01] ...........................i....................................................................i... 1600/5131
[00:46:01] ...........................i....................................................................i... 1600/5131
[00:46:05] .........................................................F.......................................... 1700/5131
[00:46:08] .......................................................................F............................ 1800/5131
[00:46:14] .....................................i.............................................................. 2000/5131
[00:46:18] .................................................................................................... 2100/5131
[00:46:21] .................................................................................................... 2200/5131
[00:46:25] .................................................................................................... 2300/5131
[00:46:25] .................................................................................................... 2300/5131
[00:46:29] .................................................................................................... 2400/5131
[00:46:32] ........................F........................................................................... 2500/5131
[00:46:40] .................................................................................................... 2700/5131
[00:46:44] .................................................................................................... 2800/5131
[00:46:46] .................................................................................................... 2900/5131
[00:46:50] .................................................................................................... 3000/5131
---
[00:47:13] ..i................................................................................................. 3900/5131
[00:47:18] .................................................................................................... 4000/5131
[00:47:23] .................................................................................................... 4100/5131
[00:47:26] .................................................................................................... 4200/5131
[00:47:29] ..........................F.............................F........................................... 4300/5131
[00:47:34] ..i................................................................................................. 4400/5131
[00:47:41] .................................................................................................... 4600/5131
[00:47:45] .......................................................................................i............ 4700/5131
[00:47:48] .................................................................................................... 4800/5131
[00:47:51] .................................................................................................... 4900/5131
[00:47:51] .................................................................................................... 4900/5131
[00:47:54] .................................................................................................... 5000/5131
[00:47:56] ......................................................................i............................. 5100/5131
t_start":19,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires finding type of Foo::X, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when finding type of Foo::X\n  --> /checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs:14:19\n   |\nLL | trait Foo<X = Box<Foo>> {\n   |                   ^^^\n   |\n   = note: ...which again requires finding type of Foo::X, completing the cycle\n\n"}
[00:47:57] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:57] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] thread '[ui] ui/cycle-trait/cycle-trait-default-type-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] 
[00:47:57] ---- [ui] ui/existential_types/no_inferrable_concrete_type.rs stdout ----
[00:47:57] diff of stderr:
[00:47:57] 
[00:47:57] - error[E0391]: cycle detected when processing `Foo`
[00:47:57] + error[E0391]: cycle detected when finding type of Foo
[00:47:57] 2   --> $DIR/no_inferrable_concrete_type.rs:16:1
[00:47:57] 3    |
[00:47:57] 4 LL |rt":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"...which requires processing `cycle1`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":739,"byte_end":764,"line_start":24,"line_end":24,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle1() -> impl Clone {","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires finding type of cycle2::{{impl-Trait}}...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":953,"byte_end":963,"line_start":33,"line_end":33,"column_start":16,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle2() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{t":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires finding type of cycle1::{{impl-Trait}}, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when finding type of cycle1::{{impl-Trait}}\n  --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:24:16\n   |\nLL | fn cycle1() -> impl Clone {\n   |                ^^^^^^^^^^\n   |\nnote: ...which requires processing `cycle1`...\n  --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:24:1\n   |\nLL | fn cycle1() -> impl Clone {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\nnote: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...\nnote: ...which requires finding type of cycle2::{{impl-Trait}}...\n  --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:33:16\n   |\nLL | fn cycle2() -> impl Clone {\n   |                ^^^^^^^^^^\nnote: ...which requires processing `cycle2`...\n  --> /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:33:1\n   |\nLL | fn cycle2() -> impl Clone {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\nnote: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...\n   = note: ...which again requires finding type of cycle1::{{impl-Trait}}, completing the cycle\n\n"}
[00:47:57] {"message":"cycle detected when finding type of cycle1::{{impl-Trait}}","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n