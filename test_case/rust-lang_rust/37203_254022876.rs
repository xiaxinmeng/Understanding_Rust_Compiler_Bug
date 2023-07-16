 diff
diff --git a/src/libpanic_unwind/gcc.rs b/src/libpanic_unwind/gcc.rs
index f9d55e0..3b0e314 100644
--- a/src/libpanic_unwind/gcc.rs
+++ b/src/libpanic_unwind/gcc.rs
@@ -252,6 +252,7 @@ unsafe extern "C" fn rust_eh_personality(state: uw::_Unwind_State,
     }
 }

+#[cfg(not(llvm_libunwind))]
 unsafe fn find_eh_action(context: *mut uw::_Unwind_Context) -> EHAction {
     let lsda = uw::_Unwind_GetLanguageSpecificData(context) as *const u8;
     let mut ip_before_instr: c_int = 0;
@@ -267,6 +268,19 @@ unsafe fn find_eh_action(context: *mut uw::_Unwind_Context) -> EHAction {
     eh::find_eh_action(lsda, &eh_context)
 }

+#[cfg(llvm_libunwind)]
+unsafe fn find_eh_action(context: *mut uw::_Unwind_Context) -> EHAction {
+    let lsda = uw::_Unwind_GetLanguageSpecificData(context) as *const u8;
+    let ip = uw::_Unwind_GetIP(context);
+    let eh_context = EHContext {
+        ip: ip,
+        func_start: uw::_Unwind_GetRegionStart(context),
+        get_text_start: &|| unimplemented!(),
+        get_data_start: &|| unimplemented!(),
+    };
+    eh::find_eh_action(lsda, &eh_context)
+}
+
 // *** Delete after a new snapshot ***
 #[cfg(all(stage0, any(target_os = "ios", not(target_arch = "arm"))))]
 #[lang = "eh_personality_catch"]
