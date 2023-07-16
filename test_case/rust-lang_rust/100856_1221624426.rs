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
Diff in /checkout/library/alloc/src/slice.rs at line 1037:
                 || (n >= 3 && runs[n - 3].len <= runs[n - 2].len + runs[n - 1].len)
                 || (n >= 4 && runs[n - 4].len <= runs[n - 3].len + runs[n - 2].len))
         {
-            if n >= 3 && runs[n - 3].len < runs[n - 1].len {
-                Some(n - 3)
-                Some(n - 2)
-            }
-            }
+            if n >= 3 && runs[n - 3].len < runs[n - 1].len { Some(n - 3) } else { Some(n - 2) }
             None
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/vec/spec_extend.rs" "/checkout/library/alloc/src/vec/partial_eq.rs" "/checkout/library/alloc/src/vec/splice.rs" "/checkout/library/alloc/src/vec/mod.rs" "/checkout/library/alloc/src/vec/is_zero.rs" "/checkout/library/alloc/src/vec/cow.rs" "/checkout/library/alloc/src/vec/spec_from_elem.rs" "/checkout/library/alloc/src/slice.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
