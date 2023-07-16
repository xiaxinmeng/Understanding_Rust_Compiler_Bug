plain
>   |
>LL | #![feature(untagged_unions)]
>   |            ^^^^^^^^^^^^^^^ feature has been removed
>   |
>   = note: unions with `Copy` and `ManuallyDrop` fields are stable; there is no intent to stabilize more
>error: aborting due to previous error
>
>For more information about this error, try `rustc --explain E0557`.
>
---
>   |
>LL | #![feature(untagged_unions)]
>   |            ^^^^^^^^^^^^^^^ feature has been removed
>   |
>   = note: unions with `Copy` and `ManuallyDrop` fields are stable; there is no intent to stabilize more
>error[E0740]: unions cannot contain fields that may need dropping
>  --> $DIR/union.rs:LL:CC
>   |
>LL |         s: S,
>LL |         s: S,
>   |         ^^^^
>   |
>   = note: a type is guaranteed not to need dropping when it implements `Copy`, or when it is the special `ManuallyDrop<_>` type
>help: when the type does not implement `Copy`, wrap it inside a `ManuallyDrop<_>` and ensure it is manually dropped
>   |
>LL |         s: std::mem::ManuallyDrop<S>,
>   |            +++++++++++++++++++++++ +
>error: aborting due to 2 previous errors
>
>Some errors have detailed explanations: E0557, E0740.
>For more information about an error, try `rustc --explain E0557`.
---

---- compile_test stdout ----
diff of stderr:

-error: you are implementing `Clone` explicitly on a `Copy` type
+error[E0557]: feature has been removed
+  --> $DIR/derive.rs:1:12
    |
    |
-LL | / impl Clone for Qux {
-LL | |     fn clone(&self) -> Self {
-LL | |         Qux
-LL | |     }
-LL | | }
-   | |_^
+LL | #![feature(untagged_unions)]
    |
    |
-   = note: `-D clippy::expl-impl-clone-on-copy` implied by `-D warnings`
-note: consider deriving `Clone` or removing `Copy`
-   |
-   |
-LL | / impl Clone for Qux {
-LL | |     fn clone(&self) -> Self {
-LL | |         Qux
-LL | |     }
-LL | | }
-   | |_^
+   = note: unions with `Copy` and `ManuallyDrop` fields are stable; there is no intent to stabilize more
 
-error: you are implementing `Clone` explicitly on a `Copy` type
-   |
-   |
-LL | / impl<'a> Clone for Lt<'a> {
-LL | |     fn clone(&self) -> Self {
-LL | |         unimplemented!()
-LL | |     }
-LL | | }
-   | |_^
-   |
-note: consider deriving `Clone` or removing `Copy`
-   |
-   |
-LL | / impl<'a> Clone for Lt<'a> {
-LL | |     fn clone(&self) -> Self {
-LL | |         unimplemented!()
-LL | |     }
-LL | | }
-   | |_^
 
 
-error: you are implementing `Clone` explicitly on a `Copy` type
-   |
-   |
-LL | / impl Clone for BigArray {
-LL | |     fn clone(&self) -> Self {
-LL | |         unimplemented!()
-LL | |     }
-LL | | }
-   | |_^
-   |
-note: consider deriving `Clone` or removing `Copy`
-   |
-   |
-LL | / impl Clone for BigArray {
-LL | |     fn clone(&self) -> Self {
-LL | |         unimplemented!()
-LL | |     }
-LL | | }
-   | |_^
-
-error: you are implementing `Clone` explicitly on a `Copy` type
-   |
-   |
-LL | / impl Clone for FnPtr {
-LL | |     fn clone(&self) -> Self {
-LL | |         unimplemented!()
-LL | |     }
-LL | | }
-   | |_^
-   |
-note: consider deriving `Clone` or removing `Copy`
-   |
-   |
-LL | / impl Clone for FnPtr {
-LL | |     fn clone(&self) -> Self {
-LL | |         unimplemented!()
-LL | |     }
-LL | | }
-   | |_^
-
-error: you are implementing `Clone` explicitly on a `Copy` type
-   |
-   |
-LL | / impl<T: Clone> Clone for Generic2<T> {
-LL | |     fn clone(&self) -> Self {
-LL | |         Self(self.0.clone())
-LL | |     }
-LL | | }
-   | |_^
-   |
-note: consider deriving `Clone` or removing `Copy`
-   |
-   |
-LL | / impl<T: Clone> Clone for Generic2<T> {
-LL | |     fn clone(&self) -> Self {
-LL | |         Self(self.0.clone())
-LL | |     }
-LL | | }
-   | |_^
-error: aborting due to 5 previous errors
-
+For more information about this error, try `rustc --explain E0557`.
 
---
To only update this specific test, also pass `--test-args derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/derive.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/derive.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-61ecef333190a996.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-ff530e80824c3a36.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-b92911696ae4394a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-aceff80e643e9fe7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-dcc59fbd39a40970.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7d13fa063f867ef0.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-46430df947b7dad0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/derive.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"feature has been removed","code":{"code":"E0557","explanation":"A feature attribute named a feature that has been removed.\n\nErroneous code example:\n\n