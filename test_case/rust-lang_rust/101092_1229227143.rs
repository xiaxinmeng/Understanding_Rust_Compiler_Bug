plain
failures:

---- [ui] src/test/ui/sanitize/memory.rs stdout ----
normalized stderr:
warning: the type `[isize; 32]` does not permit being left uninitialized
   |
   |
LL |     let r = unsafe { MaybeUninit::uninit().assume_init() };
   |                      |
   |                      this code causes undefined behavior when executed
   |                      this code causes undefined behavior when executed
   |                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: `#[warn(invalid_value)]` on by default
   = note: integers must not be uninitialized

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args sanitize/memory.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/sanitize/memory.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/a" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "sanitizer=memory" "-Zsanitizer-memory-track-origins" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/memory/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the type `[isize; 32]` does not permit being left uninitialized
   |
   |
LL |     let r = unsafe { MaybeUninit::uninit().assume_init() };
   |                      |
   |                      this code causes undefined behavior when executed
   |                      this code causes undefined behavior when executed
   |                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: `#[warn(invalid_value)]` on by default
   = note: integers must not be uninitialized

warning: 1 warning emitted
