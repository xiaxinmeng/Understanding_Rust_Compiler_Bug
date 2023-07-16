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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/builder.rs at line 1607:
         }
 
         // Only execute if it's supposed to run as default
-        if desc.default && should_run.is_really_default() {
-            self.ensure(step)
-            None
-        }
-        }
+        if desc.default && should_run.is_really_default() { self.ensure(step) } else { None }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/src/bootstrap/check.rs" "/checkout/compiler/rustc_traits/src/normalize_erasing_regions.rs" "/checkout/compiler/rustc_mir_build/src/build/matches/util.rs" "/checkout/compiler/rustc_mir_build/src/build/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/matches/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/misc.rs" "/checkout/src/bootstrap/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
