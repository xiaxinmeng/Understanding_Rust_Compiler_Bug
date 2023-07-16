plain
test [codegen] codegen/box-maybe-uninit.rs ... ok
test [codegen] codegen/debug-column-msvc.rs ... ignored
test [codegen] codegen/call-llvm-intrinsics.rs ... ok
test [codegen] codegen/c-variadic-opt.rs ... ok
test [codegen] codegen/cf-protection.rs#branch ... FAILED
test [codegen] codegen/cf-protection.rs#none ... FAILED
test [codegen] codegen/cf-protection.rs#full ... FAILED
test [codegen] codegen/cfguard-non-msvc.rs ... ok
test [codegen] codegen/dllimports/main.rs ... ignored
test [codegen] codegen/dllimports/main.rs ... ignored
test [codegen] codegen/cf-protection.rs#undefined ... FAILED
test [codegen] codegen/coercions.rs ... ok
test [codegen] codegen/cf-protection.rs#return ... FAILED
test [codegen] codegen/debug-linkage-name.rs ... ok
test [codegen] codegen/debug-compile-unit-path.rs ... ok
test [codegen] codegen/consts.rs ... ok
test [codegen] codegen/catch-unwind.rs ... ok
---
Some tests failed in compiletest suite=codegen mode=codegen host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu

failures:

---- [codegen] codegen/cf-protection.rs#branch stdout ----

error in revision `branch`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/cf-protection.rs" "-Zthreads=1" "--cfg" "branch" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.branch" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "cf-protection=branch" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.branch/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0463`.

------------------------------------------


---- [codegen] codegen/cf-protection.rs#none stdout ----

error in revision `none`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/cf-protection.rs" "-Zthreads=1" "--cfg" "none" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.none" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "cf-protection=none" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.none/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0463`.

------------------------------------------


---- [codegen] codegen/cf-protection.rs#full stdout ----

error in revision `full`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/cf-protection.rs" "-Zthreads=1" "--cfg" "full" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.full" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "cf-protection=full" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.full/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0463`.

------------------------------------------


---- [codegen] codegen/cf-protection.rs#undefined stdout ----

error in revision `undefined`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/cf-protection.rs" "-Zthreads=1" "--cfg" "undefined" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.undefined" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.undefined/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0463`.

------------------------------------------


---- [codegen] codegen/cf-protection.rs#return stdout ----

error in revision `return`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/cf-protection.rs" "-Zthreads=1" "--cfg" "return" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.return" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "cf-protection=return" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/cf-protection.return/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0463`.

------------------------------------------



failures:
    [codegen] codegen/cf-protection.rs#branch
    [codegen] codegen/cf-protection.rs#full
    [codegen] codegen/cf-protection.rs#none
    [codegen] codegen/cf-protection.rs#return
    [codegen] codegen/cf-protection.rs#undefined
test result: FAILED. 256 passed; 5 failed; 65 ignored; 0 measured; 0 filtered out; finished in 4.01s

Build completed unsuccessfully in 0:20:12
