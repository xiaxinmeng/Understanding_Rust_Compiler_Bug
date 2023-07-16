
error[E0472]: asm! is unsupported on this target
   --> library/core/src/hint.rs:143:13
    |
143 | /             asm!(
144 | |                 "/* {0} */",
145 | |                 in(reg) &mut dummy,
146 | |                 options(nostack, preserves_flags),
147 | |             );
    | |______________^

error: aborting due to previous error

[RUSTC-TIMING] core test:false 5.850
error: could not compile `core`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "mipsel-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host mipsel-unknown-linux-gnu --target mipsel-unknown-linux-gnu
