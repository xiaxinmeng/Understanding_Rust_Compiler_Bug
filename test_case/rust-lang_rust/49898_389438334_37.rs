\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-const-impl-wrong-type.rs","byte_start":562,"byte_end":565,"line_start":19,"line_end":19,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"    const BAR: i32 = -1;","highlight_start":16,"highlight_end":19}],"label":"expected u32, found i32","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-const-impl-wrong-type.rs","byte_start":495,"byte_end":498,"line_start":13,"line_end":13,"column_start":16,"column_end":19,"is_primary":false,"text":[{"text":"    const BAR: u32;","highlight_start":16,"highlight_end":19}],"label":"type in trait","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0326]: implemented const `BAR` has an incompatible type for trait\n  --> /checkout/src/test/ui/associated-const-impl-wrong-type.rs:19:16: in impl BAR\n   |\nLL |     const BAR: u32;\n   |                --- type in trait\n...\nLL |     const BAR: i32 = -1;\n   |                ^^^ expected u32, found i32\n\n"}
[00:48:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:30] {"message":"For more information about this error, try `rustc --explain E0326`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0326`.\n"}
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] thread '[ui] ui/associated-const-impl-wrong-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:48:30] 
[00:48:30] 
[00:48:30] ---- [ui] ui/associated-type-projection-from-multiple-supertraits.rs stdout ----
[00:48:30]  diff of stderr:
[00:48:30] 
[00:48:30] 1 error[E0221]: ambiguous associated type `Color` in bounds of `C`
[00:48:30] -   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:29:32
[00:48:30] +   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:29:32: in fn dent
[00:48:30] 3    |
[00:48:30] 4 LL |     type Color;
[00:48:30] 5    |     ----------- ambiguous `Color` from `Vehicle`
[00:48:30] 
[00:48:30] 11    |                                ^^^^^^^^ ambiguous associated type `Color`
[00:48:30] 12 
[00:48:30] 13 error[E0221]: ambiguous associated type `Color` in bounds of `BoxCar`
[00:48:30] -   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:33:33
[00:48:30] +   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:33:33: in fn dent_object
[00:48:30] 15    |
[00:48:30] 16 LL |     type Color;
[00:48:30] 17    |     ----------- ambiguous `Color` from `Vehicle`
[00:48:30] 
[00:48:30] 23    |                                 ^^^^^^^^^^^ ambiguous associated type `Color`
[00:48:30] 24 
[00:48:30] 25 error[E0191]: the value of the associated type `Color` (from the trait `Vehicle`) must be specified
[00:48:30] -   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:33:26
[00:48:30] +   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:33:26: in fn dent_object
[00:48:30] 27    |
[00:48:30] 28 LL | fn dent_object<COLOR>(c: BoxCar<Color=COLOR>) {
[00:48:30] 29    |                          ^^^^^^^^^^^^^^^^^^^ missing associated type `Color` value
[00:48:30] 30 
[00:48:30] 30 
[00:48:30] 31 error[E0221]: ambiguous associated type `Color` in bounds of `C`
[00:48:30] -   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:38:29
[00:48:30] +   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:38:29: in fn paint
[00:48:30] 33    |
[00:48:30] 34 LL |     type Color;
[00:48:30] 35    |     ----------- ambiguous `Color` from `Vehicle`
[00:48:30] 
[00:48:30] The actual stderr differed from the expected stderr.
[00:48:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-projection-from-multiple-supertraits.stderr
[00:48:30] To update references, run this command from build directory:
[00:48:30] To update references, run this command from build directory:
[00:48:30] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'associated-type-projection-from-multiple-supertraits.rs'
[00:48:30] 
[00:48:30] error: 1 errors occurred comparing output.
[00:48:30] status: exit code: 101
[00:48:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-projection-from-multiple-supertraits.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-projection-from-multiple-supertraits.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-projection-from-multiple-supertraits.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:30] ------------------------------------------
[00:48:30] 
[00:48:30] ------------------------------------------
[00:48:30] stderr:
[00:48:30] stderr:
[00:48:30] ------------------------------------------
[00:48:30] {"message":"ambiguous associated type `Color` in bounds of `C`","code":{"code":"E0221","explanation":"\nAn attempt was made to retrieve an associated type, but the type was ambiguous.\nFor example:\n\n