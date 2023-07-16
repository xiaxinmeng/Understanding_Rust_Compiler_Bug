plain

---- [ui] ui/check-cfg/well-known-values.rs stdout ----
diff of stderr:

7    |                   help: did you mean: `"linux"`
9    = note: `#[warn(unexpected_cfgs)]` on by default
9    = note: `#[warn(unexpected_cfgs)]` on by default
-    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, windows
+    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, watchos, windows
12 warning: unexpected `cfg` condition value
13   --> $DIR/well-known-values.rs:14:7


---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/well-known-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/well-known-values" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=values()" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/well-known-values/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/check-cfg/well-known-values.rs:7:7
   |
   |
LL | #[cfg(target_os = "linuz")]
   |                   |
   |                   |
   |                   help: did you mean: `"linux"`
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, watchos, windows
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/well-known-values.rs:14:7
   |
   |
LL | #[cfg(target_has_atomic = "0")]
   |                           |
   |                           |
   |                           help: did you mean: `"8"`
   |
   = note: expected values for `target_has_atomic` are: 128, 16, 32, 64, 8, ptr
warning: unexpected `cfg` condition value
  --> /checkout/src/test/ui/check-cfg/well-known-values.rs:21:7
   |
   |
LL | #[cfg(unix = "aa")]
   |           |
   |           help: remove the value
   |
   = note: no expected value for `unix`
