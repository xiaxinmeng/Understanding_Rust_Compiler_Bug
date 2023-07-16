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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_session/src/config.rs at line 684:
             // Single mode doesn't change how DWARF is emitted, but does add Split DWARF attributes
             // (pointing at the path which is being determined here). Use the path to the current
             // object file.
-            (SplitDebuginfo::Packed | SplitDebuginfo::Unpacked, SplitDwarfKind::Single) =>
-                Some(obj_out),
+            (SplitDebuginfo::Packed | SplitDebuginfo::Unpacked, SplitDwarfKind::Single) => {
+                Some(obj_out)
+            }
             // Split mode emits the DWARF into a different file, use that path.
-            (SplitDebuginfo::Packed | SplitDebuginfo::Unpacked, SplitDwarfKind::Split) =>
-                Some(dwo_out),
+            (SplitDebuginfo::Packed | SplitDebuginfo::Unpacked, SplitDwarfKind::Split) => {
+                Some(dwo_out)
         }
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_save_analysis/src/dumper.rs" "/checkout/compiler/rustc_save_analysis/src/sig.rs" "/checkout/compiler/rustc_save_analysis/src/span_utils.rs" "/checkout/compiler/rustc_session/src/filesearch.rs" "/checkout/compiler/rustc_session/src/parse.rs" "/checkout/compiler/rustc_session/src/output.rs" "/checkout/compiler/rustc_session/src/config.rs" "/checkout/compiler/rustc_save_analysis/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
