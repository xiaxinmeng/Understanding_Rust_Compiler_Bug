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
Diff in /checkout/compiler/rustc_arena/src/lib.rs at line 483:
 
         let initial = match iter.next() {
             None => return &mut [],
-            Some(val) => val
+            Some(val) => val,
 
         let size_hint = iter.size_hint();
         let size_hint = iter.size_hint();
Diff in /checkout/compiler/rustc_arena/src/lib.rs at line 504:
                     // the content of the SmallVec
                     unsafe {
                         let len = vec.len() + 1;
-                        let start_ptr =
-                            self.alloc_raw(Layout::array::<T>(len).unwrap()) as *mut T;
+                        let start_ptr = self.alloc_raw(Layout::array::<T>(len).unwrap()) as *mut T;
                         ptr::write(start_ptr, initial);
                         vec.as_ptr().copy_to_nonoverlapping(start_ptr.add(1), vec.len());
                         vec.set_len(0);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs" "/checkout/compiler/rustc_trait_selection/src/traits/project.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs" "/checkout/compiler/rustc_arena/src/tests.rs" "/checkout/compiler/rustc_arena/src/lib.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/outlives.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
