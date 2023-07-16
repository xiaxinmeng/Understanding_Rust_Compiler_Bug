plain
.............................................................iii........................ 13288/13368
................................................................................
failures:

---- [ui] src/test/ui/suggestions/abi-typo.rs stdout ----
error: fixed code is still producing diagnostics
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/abi-typo.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/abi-typo/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/abi-typo/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of calling convention not supported on this target
  --> /checkout/src/test/ui/suggestions/abi-typo.fixed:2:1
   |
LL | extern "stdcall" fn stcdall() {} //~ ERROR invalid ABI
   |
   = note: `#[warn(unsupported_calling_conventions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
