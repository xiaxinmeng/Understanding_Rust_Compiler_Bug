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
Diff in /checkout/src/bootstrap/native.rs at line 310:
     // Try curl, if that fail and we are on windows
     // fallback to PowerShell
     if !builder.try_run(Command::new("curl").args(&[
-            "-#",
-            "-y",
-            "-Y",
-            "-Y",
-            "10", // timeout if speed is < 10 bytes/sec for > 30 seconds
-            "--connect-timeout",
-            "30", // timeout if cannot connect within 30 seconds
-            "--retry",
-            "-Sf",
-            "-o",
-            tempfile,
-            url,
-            url,
-        ])) {
+        "-#",
+        "-y",
+        "30",
+        "-Y",
+        "10", // timeout if speed is < 10 bytes/sec for > 30 seconds
+        "--connect-timeout",
+        "30", // timeout if cannot connect within 30 seconds
+        "--retry",
+        "-Sf",
+        "-o",
+        tempfile,
+        url,
+        url,
+    ])) {
         if builder.build.build.contains("windows-msvc") {
             for _ in 0..3 {
                 if builder.try_run(Command::new("PowerShell.exe").args(&[
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/native.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
