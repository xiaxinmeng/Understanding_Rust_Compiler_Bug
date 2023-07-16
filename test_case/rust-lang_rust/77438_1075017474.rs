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
Diff in /checkout/library/alloc/src/collections/btree/map.rs at line 296:
         match root_node.search_tree::<K>(&key) {
             Found(mut kv) => Some(mem::replace(kv.key_mut(), key)),
             GoDown(handle) => {
-                VacantEntry { key, handle: Some(handle), dormant_map, alloc: &*map.alloc, _marker: PhantomData }
-                    .insert(());
+                VacantEntry {
+                    key,
+                    handle: Some(handle),
+                    dormant_map,
+                    alloc: &*map.alloc,
+                    _marker: PhantomData,
+                .insert(());
                 None
             }
         }
         }
Diff in /checkout/library/alloc/src/collections/btree/map.rs at line 1103:
         let other_iter =
             mem::replace(other, Self::new_in(ManuallyDrop::into_inner(self.alloc.clone())))
                 .into_iter();
-        let root = self.root.get_or_insert_with(||Root::new(&*self.alloc));
+        let root = self.root.get_or_insert_with(|| Root::new(&*self.alloc));
         root.append_from_sorted_iters(self_iter, other_iter, &mut self.length, &*self.alloc)
 
Diff in /checkout/library/alloc/src/collections/btree/map.rs at line 1217:
     {
     {
         let (map, dormant_map) = DormantMutRef::new(self);
         match map.root {
-            None => Vacant(VacantEntry { key, handle: None, dormant_map, alloc: &*map.alloc, _marker: PhantomData }),
+            None => Vacant(VacantEntry {
+                key,
+                dormant_map,
+                dormant_map,
+                alloc: &*map.alloc,
+                _marker: PhantomData,
+            }),
             Some(ref mut root) => match root.borrow_mut().search_tree(&key) {
-                Found(handle) => {
-                    Occupied(OccupiedEntry { handle, dormant_map, alloc: &*map.alloc, _marker: PhantomData })
-                }
+                Found(handle) => Occupied(OccupiedEntry {
+                    dormant_map,
+                    dormant_map,
+                    alloc: &*map.alloc,
+                    _marker: PhantomData,
+                }),
                 GoDown(handle) => Vacant(VacantEntry {
                     key,
                     handle: Some(handle),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/btree/map.rs" "/checkout/library/alloc/src/collections/vec_deque/mod.rs" "/checkout/library/alloc/src/collections/btree/remove.rs" "/checkout/library/alloc/src/collections/vec_deque/pair_slices.rs" "/checkout/library/alloc/src/collections/btree/testing/mod.rs" "/checkout/library/alloc/src/collections/btree/testing/crash_test.rs" "/checkout/library/alloc/src/collections/btree/node/tests.rs" "/checkout/library/alloc/src/collections/vec_deque/iter_mut.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:09
Diff in /checkout/library/alloc/src/collections/btree/map/tests.rs at line 116:
         let iter = mem::take(self).into_iter();
         let iter = mem::take(self).into_iter();
         if !iter.is_empty() {
-            self.root.insert(Root::new(&*self.alloc)).bulk_push(iter, &mut self.length, &*self.alloc);
+            self.root.insert(Root::new(&*self.alloc)).bulk_push(
+                iter,
+                &mut self.length,
+                &*self.alloc,
         }
     }
 }
