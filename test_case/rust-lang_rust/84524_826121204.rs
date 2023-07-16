plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.18
error[E0407]: method `may_have_side_effect` is not a member of trait `TrustedRandomAccess`
    |
    |
185 | /     fn may_have_side_effect() -> bool {
187 | |     }
    | |_____^ not a member of trait `TrustedRandomAccess`


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
