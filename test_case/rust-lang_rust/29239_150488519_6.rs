 patch
diff --git a/src/librustc_llvm/lib.rs b/src/librustc_llvm/lib.rs
index fe84cff..851b349 100644
--- a/src/librustc_llvm/lib.rs
+++ b/src/librustc_llvm/lib.rs
@@ -2014,7 +2014,7 @@ extern {
     pub fn LLVMInitializeX86TargetMC();
     pub fn LLVMInitializeX86AsmPrinter();
     pub fn LLVMInitializeX86AsmParser();
-    pub fn LLVMInitializeARMTargetInfo();
+/*    pub fn LLVMInitializeARMTargetInfo();
     pub fn LLVMInitializeARMTarget();
     pub fn LLVMInitializeARMTargetMC();
     pub fn LLVMInitializeARMAsmPrinter();
@@ -2034,7 +2034,7 @@ extern {
     pub fn LLVMInitializePowerPCTargetMC();
     pub fn LLVMInitializePowerPCAsmPrinter();
     pub fn LLVMInitializePowerPCAsmParser();
-
+*/
     pub fn LLVMRustAddPass(PM: PassManagerRef, Pass: *const c_char) -> bool;
     pub fn LLVMRustCreateTargetMachine(Triple: *const c_char,
                                        CPU: *const c_char,
diff --git a/src/librustc_trans/back/write.rs b/src/librustc_trans/back/write.rs
index 1fbbf82..933a262 100644
--- a/src/librustc_trans/back/write.rs
+++ b/src/librustc_trans/back/write.rs
@@ -967,7 +967,7 @@ pub unsafe fn configure_llvm(sess: &Session) {
     llvm::LLVMInitializeX86AsmPrinter();
     llvm::LLVMInitializeX86AsmParser();

-    llvm::LLVMInitializeARMTargetInfo();
+/*    llvm::LLVMInitializeARMTargetInfo();
     llvm::LLVMInitializeARMTarget();
     llvm::LLVMInitializeARMTargetMC();
     llvm::LLVMInitializeARMAsmPrinter();
@@ -990,7 +990,7 @@ pub unsafe fn configure_llvm(sess: &Session) {
     llvm::LLVMInitializePowerPCTargetMC();
     llvm::LLVMInitializePowerPCAsmPrinter();
     llvm::LLVMInitializePowerPCAsmParser();
-
+*/
     llvm::LLVMRustSetLLVMOptions(llvm_args.len() as c_int,
                                  llvm_args.as_ptr());
 }
