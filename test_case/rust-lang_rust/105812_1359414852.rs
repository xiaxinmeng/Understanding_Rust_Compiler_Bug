plain
test [codegen] src/test/codegen/lto-removes-invokes.rs ... ok

failures:

---- [codegen] src/test/codegen/no-jump-tables.rs#set stdout ----

error in revision `set`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/no-jump-tables.rs" "-Zthreads=1" "--cfg" "set" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/no-jump-tables.set" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/no-jump-tables.set/auxiliary" "--target" "x86_64-unknown-linux-gnu" "-Zno-jump-tables"
stdout: none
--- stderr -------------------------------
  |
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0463`.
------------------------------------------


---- [codegen] src/test/codegen/no-jump-tables.rs#unset stdout ----

error in revision `unset`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/no-jump-tables.rs" "-Zthreads=1" "--cfg" "unset" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/no-jump-tables.unset" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/no-jump-tables.unset/auxiliary" "--target" "x86_64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
  |
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors

