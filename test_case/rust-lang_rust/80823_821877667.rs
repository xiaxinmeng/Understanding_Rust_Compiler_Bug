plain
    Checking rustc-demangle v0.1.18
error: expected expression, found keyword `ref`
   --> library/alloc/src/borrow.rs:359:30
    |
359 |             Owned(o) if func(ref o) => Some(core::mem::replace(this, Borrowed(sub)).into_owned()),


error[E0369]: binary operation `==` cannot be applied to type `&B`
    |
    |
391 |         Cow::dedup_by(this, sub, |o| o == sub)
    |                                      - ^^ --- &B
    |                                      &B
    |
help: consider further restricting this bound
    |
    |
368 |     B: ToOwned + std::cmp::PartialEq,

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0369`.
For more information about this error, try `rustc --explain E0369`.
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:41
