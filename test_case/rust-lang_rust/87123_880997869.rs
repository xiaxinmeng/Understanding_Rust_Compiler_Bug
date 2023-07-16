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
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/error.rs at line 318:
             PointerUseAfterFree(a) => {
                 write!(f, "pointer to {} was dereferenced after this allocation got freed", a)
             }
-            PointerOutOfBounds { alloc_id, offset, size: Size::ZERO, msg, allocation_size } => write!(
-                f,
-                "{}{} has size {}, so pointer at offset {} is out-of-bounds",
-                msg,
-                alloc_id,
-                allocation_size.bytes(),
-                offset.bytes(),
-            ),
+            PointerOutOfBounds { alloc_id, offset, size: Size::ZERO, msg, allocation_size } => {
+                    f,
+                    f,
+                    "{}{} has size {}, so pointer at offset {} is out-of-bounds",
+                    msg,
+                    alloc_id,
+                    allocation_size.bytes(),
+                    offset.bytes(),
+            }
+            }
             PointerOutOfBounds { alloc_id, offset, size, msg, allocation_size } => write!(
                 f,
                 "{}{} has size {}, so pointer to {} bytes starting at offset {} is out-of-bounds",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/mir/abstract_const.rs" "/checkout/compiler/rustc_middle/src/mir/traversal.rs" "/checkout/compiler/rustc_middle/src/mir/coverage.rs" "/checkout/compiler/rustc_middle/src/mir/type_foldable.rs" "/checkout/compiler/rustc_middle/src/arena.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
