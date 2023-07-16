plain
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.18
[RUSTC-TIMING] cfg_if test:false 0.038
   Compiling panic_abort v0.0.0 (/checkout/library/panic_abort)
error[E0407]: method `may_have_side_effect` is not a member of trait `TrustedRandomAccess`
    |
    |
185 | /     fn may_have_side_effect() -> bool {
187 | |     }
    | |_____^ not a member of trait `TrustedRandomAccess`

[RUSTC-TIMING] adler test:false 0.197
[RUSTC-TIMING] adler test:false 0.197
[RUSTC-TIMING] panic_abort test:false 0.073
[RUSTC-TIMING] unwind test:false 0.093
[RUSTC-TIMING] libc test:false 0.832
[RUSTC-TIMING] compiler_builtins test:false 1.011
error[E0046]: not all trait items implemented, missing: `MAY_HAVE_SIDE_EFFECT`
    |
    |
181 | / unsafe impl<T, A: Allocator> TrustedRandomAccess for Drain<'_, T, A>
182 | | where
183 | |     T: Copy,
184 | | {
187 | |     }
188 | | }
188 | | }
    | |_^ missing `MAY_HAVE_SIDE_EFFECT` in implementation
    |
    = help: implement the missing item: `const MAY_HAVE_SIDE_EFFECT: bool = true;`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0046, E0407.
For more information about an error, try `rustc --explain E0046`.
