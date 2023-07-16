plain
+   |
+LL | #![feature(const_generics_defaults)]
+   |            ^^^^^^^^^^^^^^^^^^^^^^^
+   |
+   = note: `-D stable-features` implied by `-D warnings`
+
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct RarelyUseful {
 LL | |     field: i32,
 LL | |     last: [usize; 0],
 LL | | }
    |
    |
    = note: `-D clippy::trailing-empty-array` implied by `-D warnings`
    = help: consider annotating `RarelyUseful` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct OnlyField {
 LL | |     first_and_last: [usize; 0],
 LL | | }
    |
    |
    = help: consider annotating `OnlyField` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct GenericArrayType<T> {
 LL | |     field: i32,
 LL | |     last: [T; 0],
 LL | | }
    |
    |
    = help: consider annotating `GenericArrayType` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct OnlyAnotherAttribute {
 LL | |     field: i32,
 LL | |     last: [usize; 0],
 LL | | }
    |
    |
    = help: consider annotating `OnlyAnotherAttribute` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct OnlyADeriveAttribute {
 LL | |     field: i32,
 LL | |     last: [usize; 0],
 LL | | }
    |
    |
    = help: consider annotating `OnlyADeriveAttribute` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct ZeroSizedWithConst {
 LL | |     field: i32,
 LL | |     last: [usize; ZERO],
 LL | | }
    |
    |
    = help: consider annotating `ZeroSizedWithConst` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct ZeroSizedWithConstFunction {
 LL | |     field: i32,
 LL | |     last: [usize; compute_zero()],
 LL | | }
    |
    |
    = help: consider annotating `ZeroSizedWithConstFunction` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct ZeroSizedWithConstFunction2 {
 LL | |     field: i32,
 LL | |     last: [usize; compute_zero_from_arg(1)],
 LL | | }
    |
    |
    = help: consider annotating `ZeroSizedWithConstFunction2` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | struct ZeroSizedArrayWrapper([usize; 0]);
    |
    |
    = help: consider annotating `ZeroSizedArrayWrapper` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | struct TupleStruct(i32, [usize; 0]);
    |
    |
    = help: consider annotating `TupleStruct` with `#[repr(C)]` or another `repr` attribute
 
 error: trailing zero-sized array in a struct which is not marked with a `repr` attribute
    |
    |
 LL | / struct LotsOfFields {
 LL | |     f1: u32,
 LL | |     f2: u32,
 LL | |     f3: u32,
 ...  |
 LL | |     last: [usize; 0],
 LL | | }
    |
    |
    = help: consider annotating `LotsOfFields` with `#[repr(C)]` or another `repr` attribute
-error: aborting due to 11 previous errors
+error: aborting due to 12 previous errors
 
 
---
To only update this specific test, also pass `--test-args trailing_empty_array.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/trailing_empty_array.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/trailing_empty_array.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/trailing_empty_array.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `const_generics_defaults` has been stable since 1.59.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":50,"byte_end":73,"line_start":2,"line_end":2,"column_start":12,"column_end":35,"is_primary":true,"text":[{"text":"#![feature(const_generics_defaults)]","highlight_start":12,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `const_generics_defaults` has been stable since 1.59.0 and no longer requires an attribute to enable\n  --> tests/ui/trailing_empty_array.rs:2:12\n   |\nLL | #![feature(const_generics_defaults)]\n   |            ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
error: test failed, to rerun pass '--test compile-test'
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":90,"byte_end":151,"line_start":6,"line_end":9,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct RarelyUseful {","highlight_start":1,"highlight_end":22},{"text":"    field: i32,","highlight_start":1,"highlight_end":16},{"text":"    last: [usize; 0],","highlight_start":1,"highlight_end":22},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::trailing-empty-array` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider annotating `RarelyUseful` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:6:1\n   |\nLL | / struct RarelyUseful {\nLL | |     field: i32,\nLL | |     last: [usize; 0],\nLL | | }\n   | |_^\n   |\n   = note: `-D clippy::trailing-empty-array` implied by `-D warnings`\n   = help: consider annotating `RarelyUseful` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":153,"byte_end":205,"line_start":11,"line_end":13,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct OnlyField {","highlight_start":1,"highlight_end":19},{"text":"    first_and_last: [usize; 0],","highlight_start":1,"highlight_end":32},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `OnlyField` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:11:1\n   |\nLL | / struct OnlyField {\nLL | |     first_and_last: [usize; 0],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `OnlyField` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":207,"byte_end":271,"line_start":15,"line_end":18,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct GenericArrayType<T> {","highlight_start":1,"highlight_end":29},{"text":"    field: i32,","highlight_start":1,"highlight_end":16},{"text":"    last: [T; 0],","highlight_start":1,"highlight_end":18},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `GenericArrayType` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:15:1\n   |\nLL | / struct GenericArrayType<T> {\nLL | |     field: i32,\nLL | |     last: [T; 0],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `GenericArrayType` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":285,"byte_end":354,"line_start":21,"line_end":24,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct OnlyAnotherAttribute {","highlight_start":1,"highlight_end":30},{"text":"    field: i32,","highlight_start":1,"highlight_end":16},{"text":"    last: [usize; 0],","highlight_start":1,"highlight_end":22},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `OnlyAnotherAttribute` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:21:1\n   |\nLL | / struct OnlyAnotherAttribute {\nLL | |     field: i32,\nLL | |     last: [usize; 0],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `OnlyAnotherAttribute` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":373,"byte_end":442,"line_start":27,"line_end":30,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct OnlyADeriveAttribute {","highlight_start":1,"highlight_end":30},{"text":"    field: i32,","highlight_start":1,"highlight_end":16},{"text":"    last: [usize; 0],","highlight_start":1,"highlight_end":22},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `OnlyADeriveAttribute` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:27:1\n   |\nLL | / struct OnlyADeriveAttribute {\nLL | |     field: i32,\nLL | |     last: [usize; 0],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `OnlyADeriveAttribute` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":467,"byte_end":537,"line_start":33,"line_end":36,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct ZeroSizedWithConst {","highlight_start":1,"highlight_end":28},{"text":"    field: i32,","highlight_start":1,"highlight_end":16},{"text":"    last: [usize; ZERO],","highlight_start":1,"highlight_end":25},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `ZeroSizedWithConst` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:33:1\n   |\nLL | / struct ZeroSizedWithConst {\nLL | |     field: i32,\nLL | |     last: [usize; ZERO],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `ZeroSizedWithConst` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":622,"byte_end":710,"line_start":42,"line_end":45,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct ZeroSizedWithConstFunction {","highlight_start":1,"highlight_end":36},{"text":"    field: i32,","highlight_start":1,"highlight_end":16},{"text":"    last: [usize; compute_zero()],","highlight_start":1,"highlight_end":35},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `ZeroSizedWithConstFunction` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:42:1\n   |\nLL | / struct ZeroSizedWithConstFunction {\nLL | |     field: i32,\nLL | |     last: [usize; compute_zero()],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `ZeroSizedWithConstFunction` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":776,"byte_end":875,"line_start":50,"line_end":53,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct ZeroSizedWithConstFunction2 {","highlight_start":1,"highlight_end":37},{"text":"    field: i32,","highlight_start":1,"highlight_end":16},{"text":"    last: [usize; compute_zero_from_arg(1)],","highlight_start":1,"highlight_end":45},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `ZeroSizedWithConstFunction2` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:50:1\n   |\nLL | / struct ZeroSizedWithConstFunction2 {\nLL | |     field: i32,\nLL | |     last: [usize; compute_zero_from_arg(1)],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `ZeroSizedWithConstFunction2` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":877,"byte_end":918,"line_start":55,"line_end":55,"column_start":1,"column_end":42,"is_primary":true,"text":[{"text":"struct ZeroSizedArrayWrapper([usize; 0]);","highlight_start":1,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `ZeroSizedArrayWrapper` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:55:1\n   |\nLL | struct ZeroSizedArrayWrapper([usize; 0]);\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider annotating `ZeroSizedArrayWrapper` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":920,"byte_end":956,"line_start":57,"line_end":57,"column_start":1,"column_end":37,"is_primary":true,"text":[{"text":"struct TupleStruct(i32, [usize; 0]);","highlight_start":1,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `TupleStruct` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:57:1\n   |\nLL | struct TupleStruct(i32, [usize; 0]);\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider annotating `TupleStruct` with `#[repr(C)]` or another `repr` attribute\n\n"}
{"message":"trailing zero-sized array in a struct which is not marked with a `repr` attribute","code":{"code":"clippy::trailing_empty_array","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/trailing_empty_array.rs","byte_start":958,"byte_end":1218,"line_start":59,"line_end":77,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct LotsOfFields {","highlight_start":1,"highlight_end":22},{"text":"    f1: u32,","highlight_start":1,"highlight_end":13},{"text":"    f2: u32,","highlight_start":1,"highlight_end":13},{"text":"    f3: u32,","highlight_start":1,"highlight_end":13},{"text":"    f4: u32,","highlight_start":1,"highlight_end":13},{"text":"    f5: u32,","highlight_start":1,"highlight_end":13},{"text":"    f6: u32,","highlight_start":1,"highlight_end":13},{"text":"    f7: u32,","highlight_start":1,"highlight_end":13},{"text":"    f8: u32,","highlight_start":1,"highlight_end":13},{"text":"    f9: u32,","highlight_start":1,"highlight_end":13},{"text":"    f10: u32,","highlight_start":1,"highlight_end":14},{"text":"    f11: u32,","highlight_start":1,"highlight_end":14},{"text":"    f12: u32,","highlight_start":1,"highlight_end":14},{"text":"    f13: u32,","highlight_start":1,"highlight_end":14},{"text":"    f14: u32,","highlight_start":1,"highlight_end":14},{"text":"    f15: u32,","highlight_start":1,"highlight_end":14},{"text":"    f16: u32,","highlight_start":1,"highlight_end":14},{"text":"    last: [usize; 0],","highlight_start":1,"highlight_end":22},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider annotating `LotsOfFields` with `#[repr(C)]` or another `repr` attribute","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing zero-sized array in a struct which is not marked with a `repr` attribute\n  --> tests/ui/trailing_empty_array.rs:59:1\n   |\nLL | / struct LotsOfFields {\nLL | |     f1: u32,\nLL | |     f2: u32,\nLL | |     f3: u32,\n...  |\nLL | |     last: [usize; 0],\nLL | | }\n   | |_^\n   |\n   = help: consider annotating `LotsOfFields` with `#[repr(C)]` or another `repr` attribute\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
