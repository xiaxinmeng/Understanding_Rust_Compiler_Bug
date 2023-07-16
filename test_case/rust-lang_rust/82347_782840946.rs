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
   Compiling regex v1.4.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 10.49s
tidy check
tidy: Skipping binary file check, read-only filesystem
* 624 error codes
* highest error code: E0781
Found 436 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
* 319 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/bins.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/src/tools/tidy/src/bins.rs at line 60:
                     std::fs::remove_file(&path).expect("Deleted temp file");
                     // If the file is executable, then we assume that this
                     // filesystem does not track executability, so skip this check.
-                    return if exec { Unsupported } else { Supported }
+                    return if exec { Unsupported } else { Supported };
                 }
                 Err(e) => {
                     // If the directory is read-only or we otherwise don't have rights,
Diff in /checkout/src/tools/tidy/src/bins.rs at line 81:
         for &source_dir in sources {
             match check_dir(source_dir) {
                 Unsupported => return false,
-                ReadOnlyFs => return match check_dir(output) {
-                    Supported => true,
-                },
-                },
+                ReadOnlyFs => {
+                    return match check_dir(output) {
+                        Supported => true,
+                    };
+                }
                 _ => {}
-
-
             }
         }
 
Diff in /checkout/src/tools/tidy/src/bins.rs at line 93:
+        return true;
     }
 
 
     #[cfg(unix)]
Build completed unsuccessfully in 0:00:13
