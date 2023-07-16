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
 
-
     /// Clear the poisoned state from a mutex
     ///
     /// If the mutex is poisoned, it will remain poisoned until this function is called
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sync/mpsc/spsc_queue/tests.rs" "/checkout/library/std/src/sync/mpsc/blocking.rs" "/checkout/library/std/src/sync/mutex.rs" "/checkout/library/std/src/sync/rwlock/tests.rs" "/checkout/library/std/src/sync/barrier.rs" "/checkout/library/std/src/sync/mutex/tests.rs" "/checkout/library/std/src/sync/poison.rs" "/checkout/library/std/src/sync/mpsc/stream.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
