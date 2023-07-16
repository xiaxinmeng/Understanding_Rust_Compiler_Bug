plain
---- [codegen] tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi/auxiliary" "-Clto" "-Cno-prepopulate-passes" "-Ctarget-feature=-crt-static" "-Zsanitizer=cfi"
stdout: none
--- stderr -------------------------------
  --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:67:20
   |
52 | pub type Type1 = impl Send;
   |                  --------- the expected opaque type
   |                  --------- the expected opaque type
...
66 |     let closure1 = || { };
67 |     let _: Type1 = closure1;
   |            -----   ^^^^^^^^ expected opaque type, found closure
   |            |
   |            expected due to this
   |            expected due to this
   |
   = note: expected opaque type `Type1`
                  found closure `[closure@/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:66:20: 66:22]`
error[E0308]: mismatched types
  --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:71:20
   |
53 | pub type Type2 = impl Send;
---
   |            |
   |            expected due to this
   |
   = note:     expected opaque type `Type2`
           found struct constructor `fn(i32) -> fn1::Foo {fn1::Foo}`
error[E0308]: mismatched types
  --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:77:20
   |
54 | pub type Type3 = impl Send;
---
   |            |
   |            expected due to this
   |
   = note: expected opaque type `Type3`
                  found fn item `unsafe extern "C" fn() {fn1::foo}`
error[E0308]: mismatched types
  --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:82:24
   |
55 | pub type Type4 = impl Send;
---
   |                |
   |                expected due to this
   |
   = note: expected opaque type `Type4`
                   found struct `fn1::{closure#1}::Foo`
error[E0308]: mismatched types
  --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:95:20
   |
57 | pub type Type6 = impl Send;
57 | pub type Type6 = impl Send;
   |                  --------- the expected opaque type
...
95 |     let _: Type6 = <Struct1<i32>>::foo;
   |            -----   ^^^^^^^^^^^^^^^^^^^ expected opaque type, found fn item
   |            expected due to this
   |
   = note: expected opaque type `Type6`
   = note: expected opaque type `Type6`
                  found fn item `for<'a> fn(&'a Struct1<i32>) {fn1::<impl Struct1<i32>>::foo}`
error[E0308]: mismatched types
  --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:98:20
   |
58 | pub type Type7 = impl Send;
58 | pub type Type7 = impl Send;
   |                  --------- the expected opaque type
...
98 |     let _: Type7 = <dyn Trait1<i32>>::foo;
   |            -----   ^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found fn item
   |            expected due to this
   |
   = note: expected opaque type `Type7`
   = note: expected opaque type `Type7`
                  found fn item `for<'a> fn(&'a dyn Trait1<i32>) {<dyn Trait1<i32> as Trait1<i32>>::foo}`
error[E0308]: mismatched types
   --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:101:20
    |
59  | pub type Type8 = impl Send;
59  | pub type Type8 = impl Send;
    |                  --------- the expected opaque type
...
101 |     let _: Type8 = <i32 as Trait1<i32>>::foo;
    |            -----   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found fn item
    |            expected due to this
    |
    = note: expected opaque type `Type8`
    = note: expected opaque type `Type8`
                   found fn item `for<'a> fn(&'a i32) {<i32 as Trait1<i32>>::foo}`
error[E0308]: mismatched types
   --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:104:20
    |
60  | pub type Type9 = impl Send;
60  | pub type Type9 = impl Send;
    |                  --------- the expected opaque type
...
104 |     let _: Type9 = <Struct1<i32> as Trait1<i32>>::foo;
    |            -----   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found fn item
    |            expected due to this
    |
    = note: expected opaque type `Type9`
    = note: expected opaque type `Type9`
                   found fn item `for<'a> fn(&'a Struct1<i32>) {<Struct1<i32> as Trait1<i32>>::foo}`
error[E0308]: mismatched types
   --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:108:21
    |
61  | pub type Type10 = impl Send;
61  | pub type Type10 = impl Send;
    |                   --------- the expected opaque type
...
108 |     let _: Type10 = Qux([0; 32]);
    |            ------   ^^^^^^^^^^^^ expected opaque type, found struct `Qux`
    |            expected due to this
    |
    = note: expected opaque type `Type10`
    = note: expected opaque type `Type10`
                    found struct `Qux<{integer}, 32>`
error[E0308]: mismatched types
   --> /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs:113:21
    |
62  | pub type Type11 = impl Send;
62  | pub type Type11 = impl Send;
    |                   --------- the expected opaque type
...
113 |     let _: Type11 = Quuux;
    |            |
    |            expected due to this
    |
    = note:     expected opaque type `Type11`
    = note:     expected opaque type `Type11`
            found struct constructor `fn(&i32, &Quux<'_>) -> Quuux<'_, '_> {Quuux::<'_, '_>}`
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
