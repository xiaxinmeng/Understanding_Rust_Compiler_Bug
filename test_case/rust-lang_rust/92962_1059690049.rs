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
Diff in /checkout/library/alloc/src/collections/btree/map.rs at line 1149:
         let root_node = match map.root.as_mut() {
             Some(root) => root.borrow_mut(),
             None => {
-                return Vacant(VacantEntry { key, handle: None, dormant_map, _marker: PhantomData });
+                return Vacant(VacantEntry {
+                    key,
+                    dormant_map,
+                    dormant_map,
+                    _marker: PhantomData,
             }
         };
         };
         match root_node.search_tree(&key) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/vec_deque/into_iter.rs" "/checkout/library/alloc/src/collections/btree/set/tests.rs" "/checkout/library/alloc/src/collections/vec_deque/macros.rs" "/checkout/library/alloc/src/collections/btree/search.rs" "/checkout/library/alloc/src/collections/vec_deque/iter.rs" "/checkout/library/alloc/src/collections/btree/append.rs" "/checkout/library/alloc/src/collections/btree/map.rs" "/checkout/library/alloc/src/collections/btree/split.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
