plain

5    |                            ^^^^^^^^^^
6    |
7    = note: `#[warn(unexpected_cfgs)]` on by default
-    = note: expected values for `target_arch` are: aarch64, arm, avr, bpf, hexagon, m68k, mips, mips64, msp430, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, wasm32, wasm64, x86, x86_64
+    = note: expected values for `target_arch` are: aarch64, arm, avr, bpf, hexagon, loongarch64, m68k, mips, mips64, msp430, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, wasm32, wasm64, x86, x86_64
10 warning: 1 warning emitted
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/compact-values/compact-values.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-cfg/compact-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/compact-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/compact-values" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--check-cfg=values()" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/compact-values/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/check-cfg/compact-values.rs:11:28
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL | #[cfg(target(os = "linux", arch = "X"))]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: `#[warn(unexpected_cfgs)]` on by default
   = note: expected values for `target_arch` are: aarch64, arm, avr, bpf, hexagon, loongarch64, m68k, mips, mips64, msp430, nvptx64, powerpc, powerpc64, riscv32, riscv64, s390x, sparc, sparc64, wasm32, wasm64, x86, x86_64
warning: 1 warning emitted
------------------------------------------


