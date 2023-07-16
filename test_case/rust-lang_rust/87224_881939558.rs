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
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/error.rs at line 324:
                     f,
                     "{}{alloc_id} has size {alloc_size}, so pointer at offset {ptr_offset} is out-of-bounds",
                     msg,
-                    alloc_id=alloc_id,
-                    alloc_size=alloc_size.bytes(),
-                    ptr_offset=ptr_offset,
+                    alloc_id = alloc_id,
+                    alloc_size = alloc_size.bytes(),
+                    ptr_offset = ptr_offset,
             }
             }
             PointerOutOfBounds { alloc_id, alloc_size, ptr_offset, ptr_size, msg } => write!(
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/error.rs at line 333:
                 f,
                 "{}{alloc_id} has size {alloc_size}, so pointer to {ptr_size} byte{ptr_size_p} starting at offset {ptr_offset} is out-of-bounds",
                 msg,
-                alloc_id=alloc_id,
-                alloc_size=alloc_size.bytes(),
-                ptr_size=ptr_size.bytes(),
-                ptr_size_p=pluralize!(ptr_size.bytes()),
-                ptr_offset=ptr_offset,
+                alloc_id = alloc_id,
+                alloc_size = alloc_size.bytes(),
+                ptr_size = ptr_size.bytes(),
+                ptr_size_p = pluralize!(ptr_size.bytes()),
+                ptr_offset = ptr_offset,
             ),
             DanglingIntPointer(0, CheckInAllocMsg::InboundsTest) => {
                 write!(f, "null pointer is not a valid pointer for this operation")
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/graph_cyclic_cache.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/value.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/pointer.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/queries.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/mir/terminator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
