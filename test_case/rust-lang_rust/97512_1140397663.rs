plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_target/src/spec/abi.rs at line 12:
     // hashing tests. These are used in many places, so giving them stable values reduces test
     // churn. The specific values are meaningless.
     Rust,
-    Cdecl { unwind: bool },
-    Stdcall { unwind: bool },
-    Fastcall { unwind: bool },
-    Vectorcall { unwind: bool },
-    Vectorcall { unwind: bool },
-    Thiscall { unwind: bool },
-    Aapcs { unwind: bool },
-    Win64 { unwind: bool },
-    SysV64 { unwind: bool },
+        unwind: bool,
+    },
+    Cdecl {
+        unwind: bool,
---
+    },
+    SysV64 {
+        unwind: bool,
+    },
     PtxKernel,
     Msp430Interrupt,
     X86Interrupt,
Diff in /checkout/compiler/rustc_target/src/spec/abi.rs at line 30:
     AvrNonBlockingInterrupt,
     CCmseNonSecureCall,
-    System { unwind: bool },
+    System {
+        unwind: bool,
+    },
+    },
     RustIntrinsic,
     RustCall,
     PlatformIntrinsic,
Diff in /checkout/compiler/rustc_target/src/abi/call/mod.rs at line 577:
 pub enum Conv {
     // General language calling conventions, for which every target
     // should have its own backend (e.g. LLVM) support.
     C,
     Rust,
 
Diff in /checkout/compiler/rustc_target/src/abi/call/mod.rs at line 585:
Diff in /checkout/compiler/rustc_target/src/abi/call/mod.rs at line 585:
     RustCold,
 
     // Target-specific calling conventions.
-
     ArmAapcs,
     CCmseNonSecureCall,
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/android_base.rs" "/checkout/compiler/rustc_target/src/spec/bpf_base.rs" "/checkout/compiler/rustc_target/src/spec/wasm32_unknown_emscripten.rs" "/checkout/compiler/rustc_target/src/spec/abi.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7em_none_eabi.rs" "/checkout/compiler/rustc_target/src/spec/armv5te_unknown_linux_uclibceabi.rs" "/checkout/compiler/rustc_target/src/spec/avr_unknown_gnu_atmega328.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_uwp_windows_gnu.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
