plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between fdca237d5194bf8a1c9b437ebd2114d1c2ba6195 and 1b94979afc48224c46a6197459e88cf1db9eb029
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Diff < left / right > :
 error: any use of this value will cause an error
   --> $DIR/erroneous_const2.rs:LL:CC
    |
 LL | const FOO: u32 = [X - Y, Y - X][(X < Y) as usize];
<   |                   |
<   |                   |
<   |                   attempt to compute `5_u32 - 6_u32`, which would overflow
>   | --------------    ^^^^^ attempt to compute `5_u32 - 6_u32`, which would overflow
    |
    = note: `#[deny(const_err)]` on by default
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
 
 error[E0080]: evaluation of constant value failed
   --> $DIR/erroneous_const2.rs:LL:CC
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
 LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
    |          -                                                    ^^^ not found in this scope
    |          |
    |          help: you might be missing a type parameter: `, VAL`
 error[E0046]: not all trait items implemented, missing: `VAL`
   --> $DIR/ice-6252.rs:10:1
    |
 LL |     const VAL: T;
 LL |     const VAL: T;
-   |     --------- `VAL` from trait
+   |     ------------ `VAL` from trait
 ...
 LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `VAL` in implementation
 error: constant expression depends on a generic parameter
   --> $DIR/ice-6252.rs:13:9
    |
    |
 LL |     [1; <Multiply<Five, Five>>::VAL];
    |
    = note: this may fail depending on what value the parameter takes
 
 error: aborting due to 4 previous errors
---
To only update this specific test, also pass `--test-args crashes/ice-6252.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6252.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6252.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bb39617b33963acb.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-0fd25f6e303f34bc.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6252.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"cannot find type `PhantomData` in this scope","code":{"code":"E0412","explanation":"A used type name is not in scope.\n\nErroneous code examples:\n\n