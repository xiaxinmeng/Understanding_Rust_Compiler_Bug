plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:216:30
    |
216 |     assert_approx_eq!(1.0f32.round_to_even(), 1.0f32);
    |                              ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:217:30
    |
217 |     assert_approx_eq!(1.3f32.round_to_even(), 1.0f32);
    |                              ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:218:30
    |
218 |     assert_approx_eq!(1.5f32.round_to_even(), 2.0f32);
    |                              ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:219:30
    |
219 |     assert_approx_eq!(1.7f32.round_to_even(), 2.0f32);
    |                              ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:220:30
    |
220 |     assert_approx_eq!(2.5f32.round_to_even(), 2.0f32);
    |                              ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:221:30
    |
221 |     assert_approx_eq!(0.0f32.round_to_even(), 0.0f32);
    |                              ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:222:33
    |
222 |     assert_approx_eq!((-0.0f32).round_to_even(), -0.0f32);
    |                                 ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:223:33
    |
223 |     assert_approx_eq!((-1.0f32).round_to_even(), -1.0f32);
    |                                 ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:224:33
    |
224 |     assert_approx_eq!((-1.3f32).round_to_even(), -1.0f32);
    |                                 ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:225:33
    |
225 |     assert_approx_eq!((-1.5f32).round_to_even(), -2.0f32);
    |                                 ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:226:33
    |
226 |     assert_approx_eq!((-1.7f32).round_to_even(), -2.0f32);
    |                                 ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f32` in the current scope
   --> library/std/src/f32/tests.rs:227:33
    |
227 |     assert_approx_eq!((-2.5f32).round_to_even(), -2.0f32);
    |                                 ^^^^^^^^^^^^^ method not found in `f32`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:218:30
    |
218 |     assert_approx_eq!(1.0f64.round_to_even(), 1.0f64);
    |                              ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:219:30
    |
219 |     assert_approx_eq!(1.3f64.round_to_even(), 1.0f64);
    |                              ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:220:30
    |
220 |     assert_approx_eq!(1.5f64.round_to_even(), 2.0f64);
    |                              ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:221:30
    |
221 |     assert_approx_eq!(1.7f64.round_to_even(), 2.0f64);
    |                              ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:222:30
    |
222 |     assert_approx_eq!(2.5f64.round_to_even(), 2.0f64);
    |                              ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:223:30
    |
223 |     assert_approx_eq!(0.0f64.round_to_even(), 0.0f64);
    |                              ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:224:33
    |
224 |     assert_approx_eq!((-0.0f64).round_to_even(), -0.0f64);
    |                                 ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:225:33
    |
225 |     assert_approx_eq!((-1.0f64).round_to_even(), -1.0f64);
    |                                 ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:226:33
    |
226 |     assert_approx_eq!((-1.3f64).round_to_even(), -1.0f64);
    |                                 ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:227:33
    |
227 |     assert_approx_eq!((-1.5f64).round_to_even(), -2.0f64);
    |                                 ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:228:33
    |
228 |     assert_approx_eq!((-1.7f64).round_to_even(), -2.0f64);
    |                                 ^^^^^^^^^^^^^ method not found in `f64`

error[E0599]: no method named `round_to_even` found for type `f64` in the current scope
   --> library/std/src/f64/tests.rs:229:33
    |
229 |     assert_approx_eq!((-2.5f64).round_to_even(), -2.0f64);
    |                                 ^^^^^^^^^^^^^ method not found in `f64`
error: aborting due to 24 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "std" "-p" "unwind" "-p" "alloc" "-p" "core" "-p" "proc_macro" "-p" "panic_unwind" "-p" "panic_abort" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:58
