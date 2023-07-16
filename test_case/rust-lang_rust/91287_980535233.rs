plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 84826fec957aa17b0e068775c1c5574f707d43b0 and 32ae02513c76f924d8b0d67315bb5d17e4cd8e84
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

 error[E0412]: cannot find type `PhantomData` in this scope
   --> $DIR/ice-6252.rs:8:9
    |
 LL |     _n: PhantomData,
    |
 help: consider importing one of these items
    |
 LL | use core::marker::PhantomData;
 LL | use core::marker::PhantomData;
    |
 LL | use serde::__private::PhantomData;
 LL | use std::marker::PhantomData;
    |
 
 error[E0412]: cannot find type `VAL` in this scope
 error[E0412]: cannot find type `VAL` in this scope
   --> $DIR/ice-6252.rs:10:63
    |
error: test failed, to rerun pass '--test compile-test'
 LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
    |          -                                                    ^^^ not found in this scope
    |          |
    |          help: you might be missing a type parameter: `, VAL`
-error[E0283]: type annotations needed
-  --> $DIR/ice-6252.rs:10:12
+error[E0046]: not all trait items implemented, missing: `VAL`
+  --> $DIR/ice-6252.rs:10:1
+  --> $DIR/ice-6252.rs:10:1
    |
+LL |     const VAL: T;
+   |     ------------- `VAL` from trait
+...
 LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
-   |            ^^^^^^^^^^^^^^ cannot infer type for struct `Multiply<N, M>`
-   |
-   = note: cannot satisfy `Multiply<N, M>: TypeVal<usize>`
+   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `VAL` in implementation
 error: aborting due to 3 previous errors
 
-Some errors have detailed explanations: E0283, E0412.
-For more information about an error, try `rustc --explain E0283`.
---
To only update this specific test, also pass `--test-args crashes/ice-6252.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6252.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6252.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6252.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"cannot find type `PhantomData` in this scope","code":{"code":"E0412","explanation":"A used type name is not in scope.\n\nErroneous code examples:\n\n