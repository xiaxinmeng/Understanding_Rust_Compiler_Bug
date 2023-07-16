\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-output-format.rs","byte_start":295,"byte_end":298,"line_start":11,"line_end":11,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"    let _y = bar(); //~ ERROR use of unstable library feature","highlight_start":14,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(unstable_test_feature)] to the crate attributes to enable","code":null,"level":"help","spans":
[01:01:44] +    |
[01:01:44] +    = note: #[warn(deprecated)] on by default
[01:01:44] 14 
[01:01:44] 15 
[01:01:44] 15 
[01:01:44] 
[01:01:44] 
[01:01:44] The actual stderr differed from the expected stderr.
[01:01:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format-2/lint-output-format-2.stderr
[01:01:44] To update references, rerun the tests and pass the `--bless` flag
[01:01:44] To only update this specific test, also pass `--test-args lint/lint-output-format-2.rs`
[01:01:44] error: 1 errors occurred comparing output.
[01:01:44] status: exit code: 0
[01:01:44] status: exit code: 0
[01:01:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-output-format-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format-2/auxiliary" "-A" "unused"
[01:01:44] ------------------------------------------
[01:01:44] 
[01:01:44] ------------------------------------------
[01:01:44] stderr:
[01:01:44] stderr:
[01:01:44] ------------------------------------------
[01:01:44] {"message":"use of deprecated item 'lint_output_format::foo': text","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-output-format-2.rs","byte_start":252,"byte_end":255,"line_start":12,"line_end":12,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"    let _x = foo();","highlight_start":14,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(deprecated)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: use of deprecated item 'lint_output_format::foo': text\n  --> /checkout/src/test/ui/lint/lint-output-format-2.rs:12:14\n   |\nLL |     let _x = foo();\n   |              ^^^\n   |\n   = note: #[warn(deprecated)] on by default\n\n"}
[01:01:44] ------------------------------------------
[01:01:44] 
[01:01:44] thread '[ui] ui/lint/lint-output-format-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:01:44] 
[01:01:44] 
[01:01:44] ---- [ui] ui/resolve/issue-23305.rs stdout ----
[01:01:44] diff of stderr:
[01:01:44] 
[01:01:44] 5    |            ^^^^
[01:01:44] 6    |
[01:01:44] 7    = note: ...which again requires processing `<impl at $DIR/issue-23305.rs:5:1: 5:20>`, completing the cycle
[01:01:44] + note: cycle used when processing ``
[01:01:44] +   --> $DIR/issue-23305.rs:1:1
[01:01:44] +    |
[01:01:44] + LL | pub trait ToNbt<T> {
[01:01:44] 8 
[01:01:44] 9 error: aborting due to previous error
[01:01:44] 10 
[01:01:44] 
[01:01:44] 
[01:01:44] 
[01:01:44] The actual stderr differed from the expected stderr.
[01:01:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/issue-2330column_end":16,"is_primary":true,"text":[{"text":"impl ToNbt<Self> {}","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires processing `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:5:1: 5:20>`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing ``","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/resolve/issue-23305.rs","byte_start":0,"byte_end":18,"line_start":1,"line_end":1,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"pub trait ToNbt<T> {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:5:1: 5:20>`\n  --> /checkout/src/test/ui/resolve/issue-23305.rs:5:12\n   |\nLL | impl ToNbt<Self> {}\n   |            ^^^^\n   |\n   = note: ...which again requires processing `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:5:1: 5:20>`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/resolve/issue-23305.rs:1:1\n   |\nLL | pub trait ToNbt<T> {\n   | ^^^^^^^^^^^^^^^^^^\n\n"}
[01:01:44] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:01:44] {"message":"For more information about this error, try `rustc --ex:"}","highlight_start":1,"highlight_end":2},{"text":"","highlight_start":1,"highlight_end":1},{"text":"impl Tr<Self> for S {} // OK","highlight_start":1,"highlight_end":29},{"text":"impl<T: Tr<Self>> Tr<T> for S {} // OK","highlight_start":1,"highlight_end":39},{"text":"impl Tr for S where Self: Copy {} // OK","highlight_start":1,"highlight_end":40},{"text":"impl Tr for S where S<Self>: Copy {} // OK","highlight_start":1,"highlight_end":43},{"text":"impl Tr for S where Self::A: Copy {} // OK","highlight_start":1,"highlight_end":43},{"text":"","highlight_start":1,"highlight_end":1},{"text":"impl Tr for Self {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":45},{"text":"impl Tr for S<Self> {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":48},{"text":"impl Self {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":38},{"text":"impl S<Self> {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":41},{"text":"impl Tr<Self::A> for S {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":51},{"text":"","highlight_start":1,"highlight_end":1},{"text":"fn main() {}","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:1: 16:13>`\n  --> /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:6\n   |\nLL | impl Self {} //~ ERROR cycle detected\n   |      ^^^^\n   |\n   = note: ...which again requires processing `<impl at /checkout/src/test/ui/r0,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"#![feature(associated_type_defaults)]","highlight_start":1,"highlight_end":38},{"text":"","highlight_start":1,"highlight_end":1},{"text":"struct S<T = u8>(T);","highlight_start":1,"highlight_end":21},{"text":"trait Tr<T = u8> {","highlight_start":1,"highlight_end":19},{"text":"    type A = ();","highlight_start":1,"highlight_end":17},{"text":"}","highlight_start":1,"highlight_end":2},{"text":"","highlight_start":1,"highlight_end":1},{"text":"impl Tr<Self> for S {} // OK","highlight_start":1,"highlight_end":29},{"text":"impl<T: Tr<Self>> Tr<T> for S {} // OK","highlight_start":1,"highlight_end":39},{"text":"impl Tr for S where Self: Copy {} // OK","highlight_start":1,"highlight_end":40},{"text":"impl Tr for S where S<Self>: Copy {} // OK","highlight_start":1,"highlight_end":43},{"text":"impl Tr for S where Self::A: Copy {} // OK","highlight_start":1,"highlight_end":43},{"text":"","highlight_start":1,"highlight_end":1},{"text":"impl Tr for Self {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":45},{"text":"impl Tr for S<Self> {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":48},{"text":"impl Self {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":38},{"text":"impl S<Self> {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":41},{"text":"impl Tr<Self::A> for S {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":51},{"text":"","highlight_start":1,"highlight_end":1},{"text":"fn main() {}","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicabiliren":[{"message":"...which again requires processing `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:18:1: 18:26>`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing ``","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/resolve/resolve-self-in-impl.rs","byte_start":0,"byte_end":530,"line_start":1,"line_end":20,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"#![feature(associated_type_defaults)]","highlight_start":1,"highlight_end":38},{"text":"","highlight_start":1,"highlight_end":1},{"text":"struct S<T = u8>(T);","highlight_start":1,"highlight_end":21},{"text":"trait Tr<T = u8> {","highlight_start":1,"highlight_end":19},{"text":"    type A = ();","highlight_start":1,"highlight_end":17},{"text":"}","highlight_start":1,"highlight_end":2},{"text":"","highlight_start":1,"highlight_end":1},{"text":"impl Tr<Self> for S {} // OK","highlight_start":1,"highlight_end":29},{"text":"impl<T: Tr<Self>> Tr<T> for S {} // OK","highlight_start":1,"highlight_end":39},{"text":"impl Tr for S where Self: Copy {} // OK","highlight_start":1,"highlight_end":40},{"text":"impl Tr for S where S<Self>: Copy {} // OK","highlight_start":1,"highlight_end":43},{"text":"impl Tr for S where Self::A: Copy {} // OK","highlight_start":1,"highlight_end":43},{"text":"","highlight_start":1,"highlight_end":1},{"text":"impl Tr for Self {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":45},{"text":"impl Tr for S<Self> {} //~ ERROR cycle detected","highlight_start":1,"highlight_end":48},{"text":"impl Self {} //~ ERROR--------------------------
[01:01:44] thread '[ui] ui/resolve/resolve-self-in-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:01:44] 
[01:01:44] ---- [ui] ui/simd-intrinsic/simd-intrinsic-declaration-type.rs stdout ----
[01:01:44] diff of stderr:
[01:01:44] diff of stderr:
[01:01:44] 
[01:01:44] - error[E0442]: intrinsic argument 1 has wrong type: found `u16`, expected `i16`
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - LL |         fn x86_mm_adds_epi16(x: u16x8, y: u16x8) -> u16x8;
[01:01:44] - 
[01:01:44] - 
[01:01:44] - error[E0442]: intrinsic argument 2 has wrong type: found `u16`, expected `i16`
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - LL |         fn x86_mm_adds_epi16(x: u16x8, y: u16x8) -> u16x8;
[01:01:44] - 
[01:01:44] - 
[01:01:44] - error[E0442]: intrinsic return value has wrong type: found `u16`, expected `i16`
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - LL |         fn x86_mm_adds_epi16(x: u16x8, y: u16x8) -> u16x8;
[01:01:44] - 
[01:01:44] - 
[01:01:44] - error[E0442]: intrinsic argument 1 has wrong type: found `i16`, expected `u16`
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - LL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
[01:01:44] -    |         ^^^^^^^^^1:01:44] +    |
[01:01:44] + LL |         fn x86_mm_adds_epi16(x: u16x8, y: u16x8) -> u16x8;
[01:01:44] + 
[01:01:44] + 
[01:01:44] + error[E0442]: intrinsic return value has wrong type: found `u16`, expected `i16`
[01:01:44] +   --> $DIR/simd-intrinsic-declaration-type.rs:33:9
[01:01:44] +    |
[01:01:44] + LL |         fn x86_mm_adds_epi16(x: u16x8, y: u16x8) -> u16x8;
[01:01:44] + 
[01:01:44] + 
[01:01:44] + error[E0442]: intrinsic argument 1 has wrong type: found `i16`, expected `u16`
[01:01:44] +   --> $DIR/simd-intrinsic-declaration-type.rs:37:9
[01:01:44] +    |
[01:01:44] + LL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
[01:01:44] + 
[01:01:44] + 
[01:01:44] + error[E0442]: intrinsic argument 2 has wrong type: found `i16`, expected `u16`
[01:01:44] +   --> $DIR/simd-intrinsic-declaration-type.rs:37:9
[01:01:44] +    |
[01:01:44] + LL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
[01:01:44] + 
[01:01:44] + 
[01:01:44] + error[E0442]: intrinsic return value has wrong type: found `i16`, expected `u16`
[01:01:44] +   --> $DIR/simd-intrinsic-declaration-type.rs:37:9
[01:01:44] +    |
[01:01:44] + LL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
[01:01:44] 72 
[01:01:44] 73 error: aborting due to 12 previous errors
[01:01:44] 73 error: aborting due to 12 previous errors
[01:01:44] i16x8; // ok!\n}\n