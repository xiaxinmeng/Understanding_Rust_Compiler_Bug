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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/memory.rs at line 401:
         size: Size,
         align: Option<Align>,
         msg: CheckInAllocMsg,
-        alloc_size: impl FnOnce(
-            Size,
-            Size,
-            M::TagExtra,
-        ) -> InterpResult<'tcx, (Size, Align, T)>,
+        alloc_size: impl FnOnce(AllocId, Size, M::TagExtra) -> InterpResult<'tcx, (Size, Align, T)>,
     ) -> InterpResult<'tcx, Option<T>> {
         fn check_offset_align(offset: u64, align: Align) -> InterpResult<'static> {
             if offset % align.bytes() == 0 {
Diff in /checkout/compiler/rustc_const_eval/src/interpret/memory.rs at line 450:
                 // we want the error to be about the bounds.
                 if let Some(align) = align {
                     if M::force_int_for_alignment_check(self) {
-                        assert!(M::PointerTag::OFFSET_IS_ADDR, "ptr-to-int cast for align check should never fail");
+                        assert!(
+                            M::PointerTag::OFFSET_IS_ADDR,
+                            "ptr-to-int cast for align check should never fail"
+                        );
                         let (_, addr) = ptr.into_parts(); // we checked that offset is absolute
                         check_offset_align(addr.bytes(), align)?;
Diff in /checkout/compiler/rustc_const_eval/src/interpret/memory.rs at line 1039:
Diff in /checkout/compiler/rustc_const_eval/src/interpret/memory.rs at line 1039:
         // Destination alloc preparations and access hooks.
         let (dest_alloc, extra) = self.get_alloc_raw_mut(dest_alloc_id)?;
         let dest_range = alloc_range(dest_offset, size * num_copies);
-        M::memory_written(*tcx, extra, &mut dest_alloc.extra, (dest_alloc_id, dest_tag), dest_range)?;
+        M::memory_written(
+            *tcx,
+            &mut dest_alloc.extra,
+            (dest_alloc_id, dest_tag),
+            dest_range,
+        )?;
+        )?;
         let dest_bytes = dest_alloc
             .get_bytes_mut_ptr(&tcx, dest_range)
             .map_err(|e| e.to_interp_error(dest_alloc_id))?
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_traits/src/evaluate_obligation.rs" "/checkout/compiler/rustc_const_eval/src/util/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/visitor.rs" "/checkout/compiler/rustc_const_eval/src/interpret/util.rs" "/checkout/compiler/rustc_const_eval/src/util/call_kind.rs" "/checkout/compiler/rustc_const_eval/src/interpret/memory.rs" "/checkout/compiler/rustc_const_eval/src/util/collect_writes.rs" "/checkout/compiler/rustc_traits/src/normalize_erasing_regions.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
