
--- rustc-1.60.0-src/src/llvm-project/llvm/cmake/modules/CrossCompile.cmake.orig        2022-04-19 14:33:12.069861903 -0700
+++ rustc-1.60.0-src/src/llvm-project/llvm/cmake/modules/CrossCompile.cmake     2022-04-19 14:35:40.140571427 -0700
@@ -82,6 +82,7 @@
         -DLLVM_ENABLE_PROJECTS="${llvm_enable_projects_arg}"
         -DLLVM_EXTERNAL_PROJECTS="${llvm_external_projects_arg}"
         -DLLVM_ENABLE_RUNTIMES="${llvm_enable_runtimes_arg}"
+        -DLLVM_INCLUDE_BENCHMARKS="${LLVM_INCLUDE_BENCHMARKS}"
         ${external_project_source_dirs}
         -DLLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN="${LLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN}"
         ${build_type_flags} ${linker_flag} ${external_clang_dir}

