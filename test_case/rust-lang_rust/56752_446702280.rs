plain
travis_time:end:014d4f80:start=1544637575668964148,finish=1544637577031147502,duration=1362183354
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:35] 
[00:56:35] running 120 tests
[00:56:38] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:56:38] ..ii.i.....iiii.....
[00:56:38] 
[00:56:38]  finished in 3.425
[00:56:38] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:53] 
[00:56:53] running 119 tests
[00:57:16] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[00:57:20] i......iii.i.....ii
[00:57:20] 
[00:57:20]  finished in 27.285
[00:57:20] travis_fold:end:test_debuginfo

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:20] 
[00:57:20] running 25 tests
acro_crate_test.rs","byte_start":923,"byte_end":966,"line_start":31,"line_end":31,"column_start":1,"column_end":44,"is_primary":true,"text":[{"text":"macro_rules! unexported_macro { () => (3) }","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_macros)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused macro definition\n  --> /checkout/src/test/ui-fulldeps/auxiliary/macro_crate_test.rs:31:1\n   |\nLL | macro_rules! unexported_macro { () => (3) }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_macros)] on by default\n\n"}
[00:57:27] {"message":"the trait `syntax::ext::base::MacResult` cannot be made into an object","code":{"code":"E0038","explanation":"\nTrait objects like `Box<Trait>` can only be constructed when certain\nrequirements are satisfied by the trait in question.\n\nTrait objects are a form of dynamic dispatch and use a dynamically sized type\nfor the inner type. So, for a given trait `Trait`, when `Trait` is treated as a\ntype, as in `Box<Trait>`, the inner type is 'unsized'. In such cases the boxed\npointer is a 'fat pointer' that contains an extra pointer to a table of methods\n(among other things) for dynamic dispatch. This design mandates some\nrestrictions on the types of traits that are allowed to be used in trait\nobjects, which are collectively termed as 'object safety' rules.\n\nAttempting to create a trait object for a non object-safe trait will trigger\nthis error.\n\nThere are various rules:\n\n### The trait cannot require `Self: Sized`\n\nWhen `Trait` is treated as a type, the type does not implement the special\n`Sized` trait, because the type does not have a known size at compile time and\ncan only be accessed behind a pointer. Thus, if we have a trait like the\nfollowing:\n\n