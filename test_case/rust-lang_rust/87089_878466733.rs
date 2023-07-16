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
Diff in /checkout/compiler/rustc_mir/src/interpret/memory.rs at line 1099:
         }
 
         // now fill in all the "init" data
-        dest_alloc.mark_compressed_init_range(&compressed, alloc_range(dest.offset, size), num_copies);
+        dest_alloc.mark_compressed_init_range(
+            &compressed,
+            alloc_range(dest.offset, size),
+            num_copies,
+        );
         // copy the relocations to the destination
         dest_alloc.mark_relocation_range(relocations);
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/monomorphize/partitioning/default.rs" "/checkout/compiler/rustc_mir/src/interpret/memory.rs" "/checkout/compiler/rustc_mir/src/interpret/visitor.rs" "/checkout/compiler/rustc_mir/src/lib.rs" "/checkout/compiler/rustc_driver/src/lib.rs" "/checkout/compiler/rustc_mir/src/interpret/operator.rs" "/checkout/compiler/rustc_mir/src/util/generic_graph.rs" "/checkout/compiler/rustc_mir/src/interpret/traits.rs"` failed.
Build completed unsuccessfully in 0:00:11
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
