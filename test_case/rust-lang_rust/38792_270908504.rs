
---- [ui] ui/macros/proc_macro_derive.rs stdout ----

	ui: /checkout/src/test/ui/macros/proc_macro_derive.rs

error: auxiliary build of "/checkout/src/test/ui/macros/auxiliary/proc_macro_derive.rs" failed to compile: 

status: exit code: 101

command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/ui/macros/auxiliary/proc_macro_derive.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui --target=x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/proc_macro_derive.stage2-x86_64-unknown-linux-musl.ui.libaux --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/proc_macro_derive.stage2-x86_64-unknown-linux-musl.ui.libaux -Crpath -O

stdout:

------------------------------------------

------------------------------------------

stderr:

------------------------------------------

error[E0463]: can't find crate for `std`

error: aborting due to previous error

------------------------------------------

thread '[ui] ui/macros/proc_macro_derive.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2465

note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:

    [ui] ui/macros/proc_macro_derive.rs
