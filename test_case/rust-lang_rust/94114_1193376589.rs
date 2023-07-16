plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/alloc/src/collections/btree/map/tests.rs at line 118:
         if !iter.is_empty() {
             // SAFETY: `self.alloc` is the allocator for this `BTreeMap`.
             unsafe {
-                self.root.insert(Root::new(*self.alloc))
-                    .bulk_push(iter, &mut self.length, *self.alloc);
+                self.root.insert(Root::new(*self.alloc)).bulk_push(
+                    iter,
+                    &mut self.length,
+                    *self.alloc,
             }
         }
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/btree/map/entry.rs" "/checkout/library/alloc/src/collections/btree/map/tests.rs" "/checkout/library/alloc/src/collections/btree/navigate.rs" "/checkout/library/alloc/src/collections/btree/borrow.rs" "/checkout/library/alloc/src/collections/btree/fix.rs" "/checkout/library/alloc/src/collections/btree/set_val.rs" "/checkout/library/alloc/src/collections/binary_heap.rs" "/checkout/library/alloc/src/collections/btree/search.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
