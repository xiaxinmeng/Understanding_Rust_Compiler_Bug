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
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/std/src/panic/tests.rs at line 64:
 
     match super::catch_unwind(|| super::panic_any(count)) {
         Ok(()) => panic!("closure did not panic"),
-        Err(e) if e.is::<Arc<()>>() => {},
+        Err(e) if e.is::<Arc<()>>() => {}
         Err(_) => panic!("closure did not panic with the expected payload"),
     }
     assert!(weak.upgrade().is_none());
Diff in /checkout/library/std/src/thread/tests.rs at line 251:
 
     match thread::spawn(|| panic_any(count)).join() {
         Ok(()) => panic!("thread did not panic"),
-        Err(e) if e.is::<Arc<()>>() => {},
+        Err(e) if e.is::<Arc<()>>() => {}
         Err(_) => panic!("thread did not panic with the expected payload"),
     }
     assert!(weak.upgrade().is_none());
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/panic/tests.rs" "/checkout/library/std/src/alloc.rs" "/checkout/library/std/src/time.rs" "/checkout/library/std/src/error/tests.rs" "/checkout/library/std/src/backtrace/tests.rs" "/checkout/library/std/src/keyword_docs.rs" "/checkout/library/std/src/macros.rs" "/checkout/library/std/benches/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:167:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:167:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
