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
Diff in /checkout/library/core/src/iter/adapters/skip.rs at line 163:
     I: DoubleEndedIterator + ExactSizeIterator,
     fn next_back(&mut self) -> Option<Self::Item> {
     fn next_back(&mut self) -> Option<Self::Item> {
-        if self.len() > 0 {
-            self.iter.next_back()
-            None
-        }
-        }
+        if self.len() > 0 { self.iter.next_back() } else { None }
 
     #[inline]
Diff in /checkout/library/core/src/iter/adapters/skip.rs at line 197:
Diff in /checkout/library/core/src/iter/adapters/skip.rs at line 197:
             move |acc, x| {
                 n -= 1;
                 let r = fold(acc, x);
-                if n == 0 {
-                    ControlFlow::Break(r)
-                    ControlFlow::from_try(r)
-                }
-                }
+                if n == 0 { ControlFlow::Break(r) } else { ControlFlow::from_try(r) }
         }
 
Diff in /checkout/library/core/src/iter/adapters/skip.rs at line 208:
         let n = self.len();
         let n = self.len();
-        if n == 0 {
-            try { init }
-        } else {
-            self.iter.try_rfold(init, check(n, fold)).into_try()
-        }
+        if n == 0 { try { init } } else { self.iter.try_rfold(init, check(n, fold)).into_try() }
 
 
     fn rfold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/benches/hash/sip.rs" "/checkout/library/core/src/iter/adapters/peekable.rs" "/checkout/library/core/src/iter/traits/double_ended.rs" "/checkout/library/core/src/iter/adapters/skip.rs" "/checkout/library/core/src/iter/traits/accum.rs" "/checkout/library/core/src/iter/adapters/mod.rs" "/checkout/library/core/src/iter/traits/mod.rs" "/checkout/library/core/src/iter/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
