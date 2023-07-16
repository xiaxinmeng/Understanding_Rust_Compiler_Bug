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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/vec/mod.rs at line 2924:
                     ptr::drop_in_place(ptr::slice_from_raw_parts_mut(vec.as_mut_ptr(), vec.len))
             }
             }
-            unsafe { drop_via_slice(self); }
+            unsafe {
+                drop_via_slice(self);
         }
         }
         // RawVec handles deallocation
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/alloc/tests.rs" "/checkout/library/alloc/src/vec/drain_filter.rs" "/checkout/library/alloc/src/raw_vec.rs" "/checkout/library/alloc/src/vec/mod.rs" "/checkout/library/alloc/src/rc.rs" "/checkout/library/alloc/src/vec/spec_extend.rs" "/checkout/library/alloc/src/string.rs" "/checkout/library/alloc/src/vec/into_iter.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
