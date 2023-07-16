plain
[00:46:36] ....................................................................................................
[00:46:39] ....................................................................................................
[00:46:43] ....................................................................................................
[00:46:47] ....................i...............................................................................
[00:46:49] ..............................F...........i.........................................................
[00:46:56] ....................................................................................................
[00:47:00] ................................................................i...................................
[00:47:02] .........................................................i...
[00:47:02] failures:
[00:47:02] failures:
[00:47:02] 
[00:47:02] ---- [ui] ui/nll/user-annotations/adt-nullary-enums.rs stdout ----
[00:47:02] 
[00:47:02] 
[00:47:02] 1 error[E0597]: `c` does not live long enough
[00:47:02] -   --> $DIR/adt-nullary-enums.rs:43:45
[00:47:02] +   --> $DIR/adt-nullary-enums.rs:44:41
[00:47:02] 3    |
[00:47:02] - LL |     combine(SomeEnum::SomeVariant(Cell::new(&c)), SomeEnum::SomeOtherVariant::<Cell<&'static u32>>); //~ ERROR
[00:47:02] -    |                                             ^^ borrowed value does not live long enough
[00:47:02] + LL |         SomeEnum::SomeVariant(Cell::new(&c)),
[00:47:02] +    |                                         ^^ borrowed value does not live long enough
[00:47:02] + ...
[00:47:02] 6 LL | }
[00:47:02] 7    | - `c` dropped here while still borrowed
[00:47:02] 
[00:47:02] 
[00:47:02] 9    = note: borrowed value must be valid for the static lifetime...
[00:47:02] 10 
[00:47:02] 11 error[E0597]: `c` does not live long enough
[00:47:02] -   --> $DIR/adt-nullary-enums.rs:48:45
[00:47:02] +   --> $DIR/adt-nullary-enums.rs:52:41
[00:47:02] 13    |
[00:47:02] - LL |     combine(SomeEnum::SomeVariant(Cell::new(&c)), SomeEnum::SomeOtherVariant::<Cell<&'a u32>>); //~ ERROR
[00:47:02] -    |                                             ^^ borrowed value does not live long enough
[00:47:02] + LL |         SomeEnum::SomeVariant(Cell::new(&c)),
[00:47:02] +    |                                         ^^ borrowed value does not live long enough
[00:47:02] + ...
[00:47:02] 16 LL | }
[00:47:02] 17    | - `c` dropped here while still borrowed
[00:47:02] 
[00:47:02] 
[00:47:02] - note: borrowed value must be valid for the lifetime 'a as defined on the function body at 46:35...
[00:47:02] -   --> $DIR/adt-nullary-enums.rs:46:35
[00:47:02] + note: borrowed value must be valid for the lifetime 'a as defined on the function body at 49:35...
[00:47:02] +   --> $DIR/adt-nullary-enums.rs:49:35
[00:47:02] 21    |
[00:47:02] 22 LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
[00:47:02] 
[00:47:02] 24 
[00:47:02] 24 
[00:47:02] 25 error[E0597]: `c` does not live long enough
[00:47:02] -   --> $DIR/adt-nullary-enums.rs:58:49
[00:47:02] +   --> $DIR/adt-nullary-enums.rs:65:45
[00:47:02] 27    |
[00:47:02] - LL |         combine(SomeEnum::SomeVariant(Cell::new(&c)), SomeEnum::SomeOtherVariant::<Cell<&'a u32>>); //~ ERROR
[00:47:02] -    |                                                 ^^ borrowed value does not live long enough
[00:47:02] + LL |             SomeEnum::SomeVariant(Cell::new(&c)),
[00:47:02] +    |                                             ^^ borrowed value does not live long enough
[00:47:02] + ...
[00:47:02] 30 LL |     };
[00:47:02] 31    |     - `c` dropped here while still borrowed
[00:47:02] 
[00:47:02] 
[00:47:02] - note: borrowed value must be valid for the lifetime 'a as defined on the function body at 55:46...
[00:47:02] -   --> $DIR/adt-nullary-enums.rs:55:46
[00:47:02] + note: borrowed value must be valid for the lifetime 'a as defined on the function body at 61:46...
[00:47:02] +   --> $DIR/adt-nullary-enums.rs:61:46
[00:47:02] 35    |
[00:47:02] 36 LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
[00:47:02] 
[00:47:02] 
[00:47:02] The actual stderr differed from the expected stderr.
[00:47:02] The actual stderr differed from the expected stderr.
[00:47:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/adt-nullary-enums.stderr
[00:47:02] To update references, rerun the tests and pass the `--bless` flag
[00:47:02] To only update this specific test, also pass `--test-args nll/user-annotations/adt-nullary-enums.rs`
[00:47:02] error: 1 errors occurred comparing output.
[00:47:02] status: exit code: 1
[00:47:02] status: exit code: 1
[00:47:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/auxiliary" "-A" "unused"
[00:47:02] ------------------------------------------
[00:47:02] 
[00:47:02] ------------------------------------------
[00:47:02] stderr:
[00:47:02] stderr:
[00:47:02] ------------------------------------------
[00:47:02] {"message":"`c` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n