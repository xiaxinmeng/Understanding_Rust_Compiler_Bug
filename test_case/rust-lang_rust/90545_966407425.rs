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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/src/slice.rs at line 1172:
                     }
                     mid = SMALL_SLICE_LEN;
                 }
-                unsafe { merge(v, mid, buf_ptr, is_less); }
+                unsafe {
+                    merge(v, mid, buf_ptr, is_less);
             }
         }
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/rtstartup/rsbegin.rs" "/checkout/library/alloc/src/slice.rs" "/checkout/library/rtstartup/rsend.rs" "/checkout/library/alloc/src/raw_vec/tests.rs" "/checkout/library/alloc/src/str.rs" "/checkout/library/alloc/src/sync.rs" "/checkout/library/alloc/src/task.rs" "/checkout/library/alloc/benches/btree/set.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
