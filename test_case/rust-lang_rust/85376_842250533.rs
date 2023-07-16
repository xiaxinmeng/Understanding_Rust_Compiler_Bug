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
Diff in /checkout/compiler/rustc_mir/src/interpret/place.rs at line 1031:
     ) -> MPlaceTy<'tcx, M::PointerTag> {
         let ptr = self.memory.allocate_bytes(str.as_bytes(), kind);
         let meta = Scalar::from_machine_usize(u64::try_from(str.len()).unwrap(), self);
-        let mplace = MemPlace {
-            ptr: ptr.into(),
-            align: Align::ONE,
-            meta: MemPlaceMeta::Meta(meta),
+        let mplace =
+        let mplace =
+            MemPlace { ptr: ptr.into(), align: Align::ONE, meta: MemPlaceMeta::Meta(meta) };
 
         let layout = self.layout_of(self.tcx.mk_static_str()).unwrap();
         MPlaceTy { mplace, layout }
Diff in /checkout/compiler/rustc_mir/src/interpret/memory.rs at line 604:
 
 
     /// Return the `extra` field of the given allocation.
-    pub fn get_alloc_extra<'a>(
-        &'a self,
-        id: AllocId,
-    ) -> InterpResult<'tcx, &'a M::AllocExtra> {
+    pub fn get_alloc_extra<'a>(&'a self, id: AllocId) -> InterpResult<'tcx, &'a M::AllocExtra> {
         Ok(&self.get_raw(id)?.extra)
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/interpret/intrinsics/type_name.rs" "/checkout/compiler/rustc_mir/src/interpret/place.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_mir/src/monomorphize/mod.rs" "/checkout/compiler/rustc_mir/src/interpret/cast.rs" "/checkout/compiler/rustc_mir/src/monomorphize/polymorphize.rs" "/checkout/compiler/rustc_mir/src/interpret/memory.rs" "/checkout/compiler/rustc_mir/src/interpret/traits.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
