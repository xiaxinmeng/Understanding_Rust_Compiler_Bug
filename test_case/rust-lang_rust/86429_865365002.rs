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
Diff in /checkout/compiler/rustc_data_structures/src/sorted_map/index_map.rs at line 86:
     /// insertion order.
     pub fn get_by_key_enumerated(&'a self, key: K) -> impl '_ + Iterator<Item = (I, &V)> {
         let lower_bound = self.idx_sorted_by_item_key.partition_point(|&i| self.items[i].0 < key);
-        self.idx_sorted_by_item_key[lower_bound..]
-            .iter()
-            .map_while(move |&i| {
-                let (k, v) = &self.items[i];
-                (k == &key).then_some((i, v))
-            })
+        self.idx_sorted_by_item_key[lower_bound..].iter().map_while(move |&i| {
+            let (k, v) = &self.items[i];
+            (k == &key).then_some((i, v))
     }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/tiny_list.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map.rs" "/checkout/compiler/rustc_data_structures/src/vec_linked_list.rs" "/checkout/compiler/rustc_data_structures/src/vec_map/tests.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map/tests.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map/index_map.rs" "/checkout/compiler/rustc_data_structures/src/small_c_str.rs" "/checkout/compiler/rustc_data_structures/src/base_n/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
