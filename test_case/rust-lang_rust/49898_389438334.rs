plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:18] 
[00:47:18] running 1405 tests
[00:47:22] ..FF.FFFF.FF.FFFF...FFFFFFFFFFF.FFFFFFFFF.FFFFFF.FF..FFF.FF.....FFFFFF..F.FFFFF.F.i.F..F..F.....F...
[00:47:28] ...F...FF..FF......F.F..F..F.FF..i............FFFFFFFFFFF.FFFF..FF.FF......F..FFF.F.F.FFF........FFF
[00:47:32] FFFFFF..F.FFFFFFFFFFFF..F..FFFFFFFFFFFF...F.FFFFF....FFF.......FF..FF.F..FFFF......F.........FFFF.FF
[00:47:35] ......FFFF.FF.F.F.FFFFFFF...F.....FFF...................................FF.....F..F.FFFFFFFFFF....FF
[00:47:39] F....FF.....FF.F.F.FFFFFFFFFFFFF.FFFF....FFFF.........F..F....................F.......F.............
[00:47:42] FF..................F..F...............................F..F.F.F.........F...FF.FF..FFFFFFF.FFFF.FF.F
[00:47:48] F.FFFF...FFF.F.F.FF.FFF...FFF..FFFFFF..F..F...FFF.FF.F..F.F...FFFFFFFF.FF.FFFFFFF..FFFFF.FF.FFFFF.FF
[00:47:54] F...FFFFFFFF....F.FFF.FF.FF..FFFFF...FF.FF...FFF.FF.FFF.F.FF...FFF....F.FFF.F.FFFF.FFF.FFF.FFF......
[00:47:59] .F.FF..FFFFFF.FFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFF.FF..F.FFFFF.FF.F.FFFF...FFFFFFF.............
[00:48:05] .....F..FFF.FFFFFFFFFFF.FFF.FFFFF.FFF.FFiF.F.F.FF.FF.FFFFFFFFFFFFFFFFFFFFFF.FFF..F.FF.F.FF.FF.F.FF..
[00:48:10] FFFFFFFFFFFFFF.Fi...FFF....F..FFFFFF..F.....................FFFF.FFFFFFFFFFF.FFFFFF..F.F............
[00:48:16] ...F....F........FF.....F.F.FFF.FFFiiFF.FFF..F.F.F.FF.FF....FFFFFF.FFF.FF.F...F.....F...F......FFFFF
[00:48:22] FFFFFFFF....FFFFF.FFFFFFFFFFF..FFFFFF..FFFFFFF.F.F.F.FFF.FFF.FFFFFFFFFFF...FF..F..FFFFFFFFFFFFFFFFFF
[00:48:29] FFF..F..FFF.F....F.FF.FFF..F.F.....Fi.............FFFFFFFFF.FFFFF.FFFF.FFFF.FFFF.FFF.FFFF.F...FF..FF
[00:48:29] ....F
[00:48:30] 
[00:48:30] ---- [ui] ui/arbitrary-self-types-not-object-safe.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 
[00:48:30] 1 error[E0038]: the trait `Foo` cannot be made into an object
[00:48:30] -   --> $DIR/arbitrary-self-types-not-object-safe.rs:40:33
[00:48:30] +   --> $DIR/arbitrary-self-types-not-object-safe.rs:40:33: in fn make_foo
[00:48:30] 3    |
[00:48:30] 4 LL |     let x = Box::new(5usize) as Box<Foo>;
[00:48:30] 5    |                                 ^^^^^^^^ the trait `Foo` cannot be made into an object
[00:48:30] 
[00:48:30] 7    = note: method `foo` has a non-standard `self` type
[00:48:30] 8 
[00:48:30] 9 error[E0038]: the trait `Foo` cannot be made into an object
[00:48:30] -   --> $DIR/arbitrary-self-types-not-object-safe.rs:40:13
[00:48:30] +   --> $DIR/arbitrary-self-types-not-object-safe.rs:40:13: in fn make_foo
[00:48:30] 11    |
[00:48:30] 12 LL |     let x = Box::new(5usize) as Box<Foo>;
[00:48:30] 13    |             ^^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/arbitrary-self-types-not-object-safe.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'arbitrary-self-types-not-object-safe.rs'
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/arbitrary-self-types-not-object-safe.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/arbitrary-self-types-not-object-safe.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/arbitrary-self-types-not-object-safe.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] ------------------------------------------
[00:48:30] stderr:
[00:48:30] stderr:
[00:48:30] ------------------------------------------
[00:48:30] {"message":"the trait `Foo` cannot be made into an object","code":{"code":"E0038","explanation":"\nTrait objects like `Box<Trait>` can only be constructed when certain\nrequirements are satisfied by the trait in question.\n\nTrait objects are a form of dynamic dispatch and use a dynamically sized type\nfor the inner type. So, for a given trait `Trait`, when `Trait` is treated as a\ntype, as in `Box<Trait>`, the inner type is 'unsized'. In such cases the boxed\npointer is a 'fat pointer' that contains an extra pointer to a table of methods\n(among other things) for dynamic dispatch. This design mandates some\nrestrictions on the types of traits that are allowed to be used in trait\nobjects, which are collectively termed as 'object safety' rules.\n\nAttempting to create a trait object for a non object-safe trait will trigger\nthis error.\n\nThere are various rules:\n\n### The trait cannot require `Self: Sized`\n\nWhen `Trait` is treated as a type, the type does not implement the special\n`Sized` trait, because the type does not have a known size at compile time and\ncan only be accessed behind a pointer. Thus, if we have a trait like the\nfollowing:\n\n