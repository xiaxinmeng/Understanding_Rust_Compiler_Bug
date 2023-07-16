plain
travis_time:end:12405edc:start=1546340846976290333,finish=1546340849272997207,duration=2296706874
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:44] .................................................................................................... 500/5217
[01:00:47] ..............................i..................................................................... 600/5217
[01:00:51] .................................................................................................... 700/5217
[01:00:56] .................................................................................................... 800/5217
[01:01:01] ...........i............FF.i........................................................................ 900/5217
[01:01:04] .....................................iiiii.......................................................... 1000/5217
[01:01:10] .................................................................................................... 1200/5217
[01:01:13] .................................................................................................... 1300/5217
[01:01:13] .................................................................................................... 1300/5217
[01:01:15] ........................F........................................................................... 1400/5217
[01:01:21] ..........................................i......................................................... 1600/5217
[01:01:21] ..........................................i......................................................... 1600/5217
[01:01:24] ...........i............................................................F........................... 1700/5217
[01:01:28] ........................................................................................F........... 1800/5217
[01:01:31] ...............................................................F.................................... 1900/5217
[01:01:38] .................................................................................................... 2100/5217
[01:01:38] .................................................................................................... 2100/5217
[01:01:42] .....F...F....................................................F..................................... 2200/5217
[01:01:50] .................................................................................................... 2400/5217
[01:01:50] .................................................................................................... 2400/5217
[01:01:54] ...........................................F........................................................ 2500/5217
[01:02:02] .................................................................................................... 2700/5217
[01:02:06] .................................................................................................... 2800/5217
[01:02:09] .................................................................................................... 2900/5217
[01:02:12] .................................................................................................... 3000/5217
---
[01:02:38] ...............................i.................................................................... 3900/5217
[01:02:41] .................................................................................................... 4000/5217
[01:02:51] .................................................................................................... 4100/5217
[01:02:55] .................................................................................................... 4200/5217
[01:02:58] ..................................................................................................F. 4300/5217
[01:03:01] .............................F............................................i......................... 4400/5217
[01:03:08] .................................................................................................... 4500/5217
[01:03:11] ...........................F........................................................................ 4600/5217
[01:03:19] .................................................................................................... 4800/5217
[01:03:22] .................................................................................................... 4900/5217
[01:03:26] .................................................................................................... 5000/5217
[01:03:32] .................
[01:03:32] .................
[01:03:32] failures:
[01:03:32] 
[01:03:32] ---- [ui] ui/cycle-trait/cycle-trait-default-type-trait.rs stdout ----
[01:03:32] diff of stderr:
[01:03:32] 
[01:03:32] 5    |                   ^^^
[01:03:32] 6    |
[01:03:32] 7    = note: ...which again requires processing `Foo::X`, completing the cycle
[01:03:32] + note: cycle used when processing ``
[01:03:32] +   --> $DIR/cycle-trait-default-type-trait.rs:4:1
[01:03:32] +    |
[01:03:32] + LL | trait Foo<X = Box<Foo>> {
[01:03:32] 8 
[01:03:32] 9 error: aborting due to previous error
[01:03:32] 10 
[01:03:32] 
[01:03:32] 
[01:03:32] 
[01:03:32] The actual stderr differed from the expected stderr.
[01:03:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/cycle-trait-default-type-trait.stderr
[01:03:32] To update references, rerun the tests and pass the `--bless` flag
[01:03:32] To only update this specific test, also pass `--test-args cycle-trait/cycle-trait-default-type-trait.rs`
[01:03:32] error: 1 errors occurred comparing output.
[01:03:32] status: exit code: 1
[01:03:32] status: exit code: 1
[01:03:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/ault-type-trait.rs","byte_start":101,"byte_end":124,"line_start":4,"line_end":4,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"trait Foo<X = Box<Foo>> {","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `Foo::X`\n  --> /checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs:4:19\n   |\nLL | trait Foo<X = Box<Foo>> {\n   |                   ^^^\n   |\n   = note: ...which again requires processing `Foo::X`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs:4:1\n   |\nLL | trait Foo<X = Box<Foo>> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:03:32] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:03:32] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:03:32] ------------------------------------------
[01:03:32] 
[01:03:32] thread '[ui] ui/cycle-trait/cycle-trait-default-type-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:03:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:32] 
[01:03:32] ---- [ui] ui/cycle-trait/cycle-trait-supertrait-direct.rs stdout ----
[01:03:32] diff of stderr:
[01:03:32] 
[01:03null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `Chromosome`\n  --> /checkout/src/test/ui/cycle-trait/cycle-trait-supertrait-direct.rs:3:19\n   |\nLL | trait Chromosome: Chromosome {\n   |                   ^^^^^^^^^^\n   |\n   = note: ...which again requires computing the supertraits of `Chromosome`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/cycle-trait/cycle-trait-supertrait-direct.rs:3:1\n   |\nLL | trait Chromosome: Chromosome {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:03:32] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:03:32] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:03:32] ------------------------------------------
[01:03:32] 
[01:03:32] thread '[ui] ui/cycle-trait/cycle-trait-supertrait-direct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:03:32] 
[01:03:32] 
[01:03:32] ---- [ui] ui/existential_types/no_inferrable_concrete_type.rs stdout ----
[01:03:32] diff of stderr:
[01:03:32] 
[01:03:32] 10 LL | fn bar(x: Foo) -> Foo { x }
[01:03:32] 11    |                       ^^^^^
[01:03:32] 12    = note: ...which again requires processing `Foo`, completing the cycle
[01:03:32] + note: cycle used when processing ``
[01:03:32] +   --> $DIR/no_inferrable_concrete_type.rs:4:1
[01:03:32null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":287,"byte_end":297,"line_start":14,"line_end":14,"column_start":16,"column_end":26,"is_primary":false,"text":[{"text":"fn cycle1() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":287,"byte_end":297,"line_start":14,"line_end":14,"column_start":16,"column_end":26,"is_primary":false,"text":[{"text":"fn cycle1() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"...which requires processing `cycle1`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":272,"byte_end":297,"line_start":14,"line_end":14,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle1() -> impl Clone {","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// ignore-tidy-linelength","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires processing `cycle2::{{impl-Trait}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":486,"byte_end":496,"line_start":23,"line_end":23,"column_start":16,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle2() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":486,"byte_end":496,"line_start":23,"line_end":23,"column_start":16,"column_end":26,"is_primary":false,"text":[{"text":"fn cycle2() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":486,"byte_end":496,"line_start":23,"line_end":23,"column_start":16,"column_end":26,"is_primary":false,"text":[{"text":"fn cycle2() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null},{"message":"...which requires processing `cycle2`...","code":null,"level":"note","spans":[{"file_name"ge":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:03:32] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:03:32] ------------------------------------------
[01:03:32] 
[01:03:32] thread '[ui] ui/issues/issue-12511.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:03:32] 
[01:03:32] 
[01:03:32] ---- [ui] ui/issues/issue-20772.rs stdout ----
[01:03:32] diff of stderr:
[01:03:32] 
[01:03:32] 8    | |__^
[01:03:32] 9    |
[01:03:32] 10    = note: ...which again requires computing the supertraits of `T`, completing the cycle
[01:03:32] + note: cycle used when processing ``
[01:03:32] +   --> $DIR/issue-20772.rs:1:1
[01:03:32] +    |
[01:03:32] + LL | / trait T : Iterator<Item=Self::Item>
[01:03:32] + LL | | //~^ ERROR cycle detected
[01:03:32] + LL | | //~| ERROR associated type `Item` not found for `Self`
[01:03:32] + LL | | {}
[01:03:32] 11 
[01:03:32] 11 
[01:03:32] 12 error[E0220]: associated type `Item` not found for `Self`
[01:03:32] 
[01:03:32] 
[01:03:32] The actual stderr differed from the expected stderr.
[01:03:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/issue-20772.stderr
[01:03:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/issue-20772.stderr
[01:03:32] To update references, rerun the tests and pass the `--bless` flag
[01:03:32] To only update this specific tes"highlight_end":26},{"text":"//~| ERROR associated type `Item` not found for `Self`","highlight_start":1,"highlight_end":55},{"text":"{}","highlight_start":1,"highlight_end":3}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires computing the supertraits of `T`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing ``","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-20772.rs","byte_start":0,"byte_end":119,"line_start":1,"line_end":4,"column_start":1,"column_end":3,"is_primary":true,"text":[{"text":"trait T : Iterator<Item=Self::Item>","highlight_start":1,"highlight_end":36},{"text":"//~^ ERROR cycle detected","highlight_start":1,"highlight_end":26},{"text":"//~| ERROR associated type `Item` not found for `Self`","highlight_start":1,"highlight_end":55},{"text":"{}","highlight_start":1,"highlight_end":3}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `T`\n  --> /checkout/src/test/ui/issues/issue-20772.rs:1:1\n   |\nLL | / trait T : Iterator<Item=Self::Item>\nLL | | //~^ ERROR cycle detected\nLL | | //~| ERROR associated type `Item` not found for `Self`\nLL | | {}\n   | |__^\n   |\n   = note: ...which again requires computing the supertraits of `T`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/issues/issue-20772.rs:1:1\n   |\nLL | / trair saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20825/issue-20825.stderr
[01:03:32] To update references, rerun the tests and pass the `--bless` flag
[01:03:32] To only update this specific test, also pass `--test-args issues/issue-20825.rs`
[01:03:32] error: 1 errors occurred comparing output.
[01:03:32] status: exit code: 1
[01:03:32] status: exit code: 1
[01:03:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20825.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20825/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20825/auxiliary" "-A" "unused"
[01:03:32] ------------------------------------------
[01:03:32] 
[01:03:32] ------------------------------------------
[01:03:32] stderr:
[01:03:32] stderr:
[01:03:32] ------------------------------------------
[01:03:32] {"message":"cycle detected when computing the supertraits of `Processor`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n