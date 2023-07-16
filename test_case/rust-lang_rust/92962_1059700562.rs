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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/collections/btree/map.rs at line 1153:
                     key,
                     handle: None,
                     dormant_map,
-                    _marker: PhantomData
+                    _marker: PhantomData,
             }
         };
         };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/vec_deque/iter.rs" "/checkout/library/alloc/src/collections/btree/append.rs" "/checkout/library/alloc/src/collections/btree/map.rs" "/checkout/library/alloc/src/collections/btree/remove.rs" "/checkout/library/alloc/src/collections/binary_heap.rs" "/checkout/library/alloc/src/collections/btree/map/tests.rs" "/checkout/library/alloc/src/collections/btree/map/entry.rs" "/checkout/library/alloc/src/collections/btree/search.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
