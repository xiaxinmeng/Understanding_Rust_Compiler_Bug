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
Diff in /checkout/src/bootstrap/dist.rs at line 2124:
         tarball.set_overlay(OverlayKind::LLVM);
 
         let src_bindir = builder.llvm_out(target).join("bin");
-        for bin in &[
-            "llvm-config",
-            "llvm-ar",
-            "llvm-objdump",
-            "llvm-bcanalyzer",
-            "llvm-dwp",
-        ] {
+        for bin in &["llvm-config", "llvm-ar", "llvm-objdump", "llvm-bcanalyzer", "llvm-dwp"] {
             tarball.add_file(src_bindir.join(exe(bin, target)), "bin", 0o755);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/lib.rs"` failed.
         }
         tarball.add_file(&builder.llvm_filecheck(target), "bin", 0o755);
Diff in /checkout/src/bootstrap/lib.rs at line 170:
 pub use crate::flags::Subcommand;
 
 const LLVM_TOOLS: &[&str] = &[
-    "llvm-nm",       // used to inspect binaries; it shows symbol names, their sizes and visibility
-    "llvm-objcopy",  // used to transform ELFs into binary format which flashing tools consume
-    "llvm-objdump",  // used to disassemble programs
-    "llvm-readobj",  // used to get information from ELFs/objects that the other tools don't provide
-    "llvm-size",     // used to prints the size of the linker sections of a program
-    "llvm-strip",    // used to discard symbols from binary files to reduce their size
-    "llvm-ar",       // used for creating and modifying archive files
-    "llvm-as",       // used to convert LLVM assembly to LLVM bitcode
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-    "llvm-dis",      // used to disassemble LLVM bitcode
-    "llc",           // used to compile LLVM bytecode
-    "opt",           // used to optimize LLVM bytecode
+    "llvm-nm", // used to inspect binaries; it shows symbol names, their sizes and visibility
+    "llvm-objcopy", // used to transform ELFs into binary format which flashing tools consume
+    "llvm-objdump", // used to disassemble programs
+    "llvm-readobj", // used to get information from ELFs/objects that the other tools don't provide
+    "llvm-size", // used to prints the size of the linker sections of a program
+    "llvm-strip", // used to discard symbols from binary files to reduce their size
+    "llvm-ar", // used for creating and modifying archive files
+    "llvm-as", // used to convert LLVM assembly to LLVM bitcode
+    "llvm-dis", // used to disassemble LLVM bitcode
+    "llc",     // used to compile LLVM bytecode
+    "opt",     // used to optimize LLVM bytecode
 
 
 const LLVM_COVERAGE_TOOLS: &[&str] = &[
