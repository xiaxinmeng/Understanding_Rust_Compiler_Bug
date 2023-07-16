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
Diff in /checkout/library/test/src/lib.rs at line 294:
     fn calc_timeout(timeout_queue: &VecDeque<TimeoutEntry>) -> Option<Duration> {
         timeout_queue.front().map(|&TimeoutEntry { timeout: next_timeout, .. }| {
             let now = Instant::now();
-            if next_timeout >= now {
-                next_timeout - now
-                Duration::new(0, 0)
-            }
-            }
+            if next_timeout >= now { next_timeout - now } else { Duration::new(0, 0) }
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/rtstartup/rsbegin.rs" "/checkout/library/test/src/term.rs" "/checkout/library/test/src/bench.rs" "/checkout/library/test/src/event.rs" "/checkout/library/test/src/helpers/exit_code.rs" "/checkout/library/test/src/lib.rs" "/checkout/library/test/src/term/win.rs" "/checkout/library/test/src/cli.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/library/std/src/io/mod.rs at line 2381:
     }
 
 
     fn consume(&mut self, amt: usize) {
-        if !self.done_first {
-            self.first.consume(amt)
-        } else {
-            self.second.consume(amt)
-        }
+        if !self.done_first { self.first.consume(amt) } else { self.second.consume(amt) }
 }
 
