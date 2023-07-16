
[00:48:17] ---- [codegen] codegen/remap_path_prefix/main.rs stdout ----
[00:48:17] 	
[00:48:17] error: verification with 'FileCheck' failed
[00:48:17] status: exit code: 1
[00:48:17] command: /usr/lib/llvm-3.7/bin/FileCheck -input-file=/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main.ll /checkout/src/test/codegen/remap_path_prefix/main.rs
[00:48:17] stdout:
[00:48:17] ------------------------------------------
[00:48:17] 
[00:48:17] ------------------------------------------
[00:48:17] stderr:
[00:48:17] ------------------------------------------
[00:48:17] /checkout/src/test/codegen/remap_path_prefix/main.rs:28:11: error: expected string not found in input
[00:48:17] // CHECK: !DIFile(filename: "/the/src/remap_path_prefix/main.rs", directory: "/the/cwd")
[00:48:17]           ^
[00:48:17] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main.ll:9:1: note: scanning from here
[00:48:17] @__rustc_debug_gdb_scripts_section__ = internal unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1
[00:48:17] ^
[00:48:17] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main.ll:44:6: note: possible intended match here
[00:48:17] !1 = !DIFile(filename: "/the/src/remap_path_prefix/main.rs", directory: "/the/cwd/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix")
[00:48:17]      ^
[00:48:17] 
[00:48:17] ------------------------------------------
[00:48:17] 
[00:48:17] thread '[codegen] codegen/remap_path_prefix/main.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2480
[00:48:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:17] 
[00:48:17] 
[00:48:17] failures:
[00:48:17]     [codegen] codegen/remap_path_prefix/main.rs
