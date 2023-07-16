plain
use std::arch::asm;

fn main() {
    unsafe {
        asm!("", options(nomem, ));
        //~^ ERROR the `nomem` option was already provided
        asm!("", options(preserves_flags, ));
        //~^ ERROR the `preserves_flags` option was already provided
        asm!("", options(nostack, preserves_flags), options());
        //~^ ERROR the `nostack` option was already provided
        asm!("", options(nostack, ), options(), options());
        //~^ ERROR the `nostack` option was already provided
        //~| ERROR the `nostack` option was already provided
        //~| ERROR the `nostack` option was already provided
        asm!(
            "",
            options(nomem, noreturn),
            options(preserves_flags, ), //~ ERROR the `noreturn` option was already provided
            options( nostack),            //~ ERROR the `nomem` option was already provided
            options(),                  //~ ERROR the `noreturn` option was already provided
    }
}
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu




The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/duplicate-options/duplicate-options.fixed
To only update this specific test, also pass `--test-args asm/aarch64/duplicate-options.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/duplicate-options.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/duplicate-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/duplicate-options/auxiliary"
stdout: none
--- stderr -------------------------------
error: the `nomem` option was already provided
   |
   |
LL |         asm!("", options(nomem, nomem));
   |                                 ^^^^^ this option was already provided

error: the `preserves_flags` option was already provided
   |
   |
LL |         asm!("", options(preserves_flags, preserves_flags));
   |                                           ^^^^^^^^^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, preserves_flags), options(nostack));
   |                                                             ^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
   |                                   ^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
   |                                                     ^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
   |                                                                       ^^^^^^^ this option was already provided

error: the `noreturn` option was already provided
   |
   |
LL |             options(preserves_flags, noreturn), //~ ERROR the `noreturn` option was already provided
   |                                      ^^^^^^^^ this option was already provided

error: the `nomem` option was already provided
   |
   |
LL |             options(nomem, nostack),            //~ ERROR the `nomem` option was already provided
   |                     ^^^^^ this option was already provided

error: the `noreturn` option was already provided
   |
   |
LL |             options(noreturn),                  //~ ERROR the `noreturn` option was already provided
   |                     ^^^^^^^^ this option was already provided
error: aborting due to 9 previous errors
------------------------------------------


