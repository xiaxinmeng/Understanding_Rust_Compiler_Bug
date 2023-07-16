plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/library/core/src/cell.rs at line 666:
         RefCell {
             value: UnsafeCell::new(value),
             borrow: Cell::new(UNUSED),
-            #[cfg(feature = "debug_refcell")] first_caller: Cell::new(None),
+            #[cfg(feature = "debug_refcell")]
+            first_caller: Cell::new(None),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/cell.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
     }
 
Diff in /checkout/library/core/src/cell.rs at line 806:
Diff in /checkout/library/core/src/cell.rs at line 806:
         #[cfg(feature = "debug_refcell")]
         match self.try_borrow() {
             Ok(b) => return b,
-            Err(e) => panic!("already mutably borrowed: {:?} at {}", e, self.first_caller.get().unwrap())
+            Err(e) => {
+                panic!("already mutably borrowed: {:?} at {}", e, self.first_caller.get().unwrap())
         }
 
 
         #[cfg(not(feature = "debug_refcell"))]
Diff in /checkout/library/core/src/cell.rs at line 900:
         #[cfg(feature = "debug_refcell")]
         match self.try_borrow_mut() {
             Ok(b) => return b,
-            Err(e) => panic!("already borrowed: {:?} at {}", e, self.first_caller.get().unwrap())
+            Err(e) => panic!("already borrowed: {:?} at {}", e, self.first_caller.get().unwrap()),
 
 
         #[cfg(not(feature = "debug_refcell"))]
Build completed unsuccessfully in 0:00:16
