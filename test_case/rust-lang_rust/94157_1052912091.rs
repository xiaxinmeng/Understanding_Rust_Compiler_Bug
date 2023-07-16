plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
# We don't compile `opaque` with either optimizations or instrumentation.
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights  -Clink-args=-fuse-ld=gold opaque.rs || exit 1
# Compile the test program with instrumentation
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/prof_data_dir" || exit 1
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights  -Clink-args=-fuse-ld=gold interesting.rs \
 -Cprofile-generate="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/prof_data_dir" -O -Ccodegen-units=1 || exit 1
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights  -Clink-args=-fuse-ld=gold main.rs -Cprofile-generate="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/prof_data_dir" -O || exit 1
# The argument below generates to the expected branch weights
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/main aaaaaaaaaaaa2bbbbbbbbbbbb2bbbbbbbbbbbbbbbbcc || exit 1
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-profdata" merge \
 -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/prof_data_dir/merged.profdata" \
 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/prof_data_dir" || exit 1
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights  -Clink-args=-fuse-ld=gold interesting.rs \
 -Cprofile-use="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/prof_data_dir/merged.profdata" -O \
 -Ccodegen-units=1 --emit=llvm-ir || exit 1
cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/interesting.ll" | "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" filecheck-patterns.txt
--- stderr -------------------------------
--- stderr -------------------------------
filecheck-patterns.txt:5:8: error: CHECK: expected string not found in input
CHECK: define void @function_called_twice(i32 %c) {{.*}} !prof [[function_called_twice_id:![0-9]+]] {
<stdin>:1:1: note: scanning from here
<stdin>:1:1: note: scanning from here
; ModuleID = 'interesting.8805ca9a-cgu.0'
<stdin>:7:1: note: possible intended match here
<stdin>:7:1: note: possible intended match here
define void @function_called_twice(i32 noundef %c) unnamed_addr #0 !prof !37 {

Input file: <stdin>
Check file: filecheck-patterns.txt


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'interesting.8805ca9a-cgu.0' 
check:5'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "interesting.8805ca9a-cgu.0" 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-unknown-linux-gnu" 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:5'0     ~
           6: ; Function Attrs: inlinehint noinline nonlazybind uwtable 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: define void @function_called_twice(i32 noundef %c) unnamed_addr #0 !prof !37 { 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:5'1     ?                                                                               possible intended match
           8: start: 
check:5'0     ~~~~~~~
           9:  %0 = icmp eq i32 %c, 50 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          10:  br i1 %0, label %bb1, label %bb3, !prof !38 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          11:  
check:5'0     ~
          12: bb1: ; preds = %start 
check:5'0     ~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
>>>>>>
make: *** [Makefile:29: all] Error 1



failures:
