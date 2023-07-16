plain
................iii..................................................................... 13112/13147
...................................
failures:

---- [ui] src/test/ui/coercion/just-metadata-bad-coercions.rs stdout ----


15 LL |     let _: JustMetadata<dyn Trait> = struct_metadata;
16    |                                      ^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `Struct`
-    = note: required for the cast to the object type `dyn Trait`
+    = note: required for the cast from `Struct` to the object type `dyn Trait`
19 
20 error[E0308]: mismatched types
---
To only update this specific test, also pass `--test-args coercion/just-metadata-bad-coercions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/just-metadata-bad-coercions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/just-metadata-bad-coercions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/just-metadata-bad-coercions/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/coercion/just-metadata-bad-coercions.rs:12:37
   |
   |
LL |     let _: JustMetadata<[Struct]> = struct_metadata; //~ ERROR mismatched types
   |            ----------------------   ^^^^^^^^^^^^^^^ expected slice, found struct `Struct`
   |            expected due to this
   |
   |
   = note: expected struct `JustMetadata<[Struct]>`
              found struct `JustMetadata<Struct>`
error[E0277]: the trait bound `Struct: Trait` is not satisfied
  --> /checkout/src/test/ui/coercion/just-metadata-bad-coercions.rs:11:38
   |
   |
LL |     let _: JustMetadata<dyn Trait> = struct_metadata; //~ ERROR `Struct: Trait` is not satisfied
   |                                      ^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `Struct`
   = note: required for the cast from `Struct` to the object type `dyn Trait`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/coercion/just-metadata-bad-coercions.rs:16:34
  --> /checkout/src/test/ui/coercion/just-metadata-bad-coercions.rs:16:34
   |
LL |     let _: JustMetadata<[u32]> = array_metadata; //~ ERROR mismatched types
   |            -------------------   ^^^^^^^^^^^^^^ expected slice `[u32]`, found array `[u8; 4]`
   |            expected due to this
   |
   |
   = note: expected struct `JustMetadata<[u32]>`
              found struct `JustMetadata<[u8; 4]>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/coercion/just-metadata-bad-coercions.rs:17:35
   |
   |
LL |     let _: JustMetadata<Struct> = array_metadata; //~ ERROR mismatched types
   |            --------------------   ^^^^^^^^^^^^^^ expected struct `Struct`, found array `[u8; 4]`
   |            expected due to this
   |
   |
   = note: expected struct `JustMetadata<Struct>`
              found struct `JustMetadata<[u8; 4]>`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
