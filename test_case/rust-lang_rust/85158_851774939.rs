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
Diff in /checkout/compiler/rustc_mir/src/transform/large_enums.rs at line 87:
                                 vec![0; std::mem::size_of::<usize>() * num_variants as usize];
                             let mut curr = 0;
                             let mut curr = 0;
-                            for byte in sizes.iter().flat_map(|sz| sz.bytes().to_ne_bytes()).take(data.len()) {
-                              data[curr] = byte;
-                              curr += 1;
+                            for byte in sizes
+                                .iter()
+                                .flat_map(|sz| sz.bytes().to_ne_bytes())
+                                .take(data.len())
+                            {
+                                data[curr] = byte;
+                                curr += 1;
                             let alloc = interpret::Allocation::from_bytes(
                                 data,
                                 data,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/deduplicate_blocks.rs" "/checkout/compiler/rustc_mir/src/transform/large_enums.rs" "/checkout/compiler/rustc_mir/src/transform/remove_storage_markers.rs" "/checkout/compiler/rustc_mir/src/transform/dest_prop.rs" "/checkout/compiler/rustc_mir/src/transform/match_branches.rs" "/checkout/compiler/rustc_mir/src/transform/validate.rs" "/checkout/compiler/rustc_mir/src/transform/function_item_references.rs" "/checkout/compiler/rustc_mir/src/transform/dump_mir.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
