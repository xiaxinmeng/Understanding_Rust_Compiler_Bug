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
Diff in /checkout/library/core/benches/slice.rs at line 126:
         #[bench]
         fn $fn(b: &mut Bencher) {
             let mut x = (0usize..$n).map(&$mapper).collect::<Vec<_>>();
-            let mut y = ($n..($n*2)).map(&$mapper).collect::<Vec<_>>();
+            let mut y = ($n..($n * 2)).map(&$mapper).collect::<Vec<_>>();
             let mut skip = 0;
             b.iter(|| {
                 for _ in 0..32 {
Diff in /checkout/library/core/benches/slice.rs at line 133:
-                    x[skip..].swap_with_slice(&mut y[..($n-skip)]);
+                    x[skip..].swap_with_slice(&mut y[..($n - skip)]);
                     skip = black_box(skip + 1) % 8;
                 }
-                black_box((x[$n/3].clone(), y[$n*2/3].clone()))
+                black_box((x[$n / 3].clone(), y[$n * 2 / 3].clone()))
         }
     };
     };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/benches/slice.rs" "/checkout/library/core/benches/ascii.rs" "/checkout/library/core/benches/ops.rs" "/checkout/library/core/benches/fmt.rs" "/checkout/library/core/benches/pattern.rs" "/checkout/library/core/benches/lib.rs" "/checkout/library/core/benches/ascii/is_ascii.rs" "/checkout/library/std/benches/hash/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
