plain
failures:

---- [codegen] src/test/codegen/naked-noinline.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-noinline/naked-noinline.ll" "/checkout/src/test/codegen/naked-noinline.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/naked-noinline.rs:31:11: error: CHECK: expected string not found in input
// CHECK: attributes [[ATTR]] = { naked noinline{{.*}} }
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-noinline/naked-noinline.ll:17:2: note: scanning from here
 ret void
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-noinline/naked-noinline.ll:17:2: note: with "ATTR" equal to "#0"
 ret void
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-noinline/naked-noinline.ll:20:1: note: possible intended match here
attributes #0 = { naked nocf_check noinline nounwind nonlazybind uwtable "branch-target-enforcement"="false" "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-noinline/naked-noinline.ll
Check file: /checkout/src/test/codegen/naked-noinline.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           12: 
           13: ; Function Attrs: nounwind nonlazybind uwtable
           14: define void @g() unnamed_addr #1 {
           15: start:
           16:  tail call void @f()
           17:  ret void
check:31'0      X~~~~~~~ error: no match found
check:31'1               with "ATTR" equal to "#0"
           18: }
check:31'0     ~
           19: 
check:31'0     ~
           20: attributes #0 = { naked nocf_check noinline nounwind nonlazybind uwtable "branch-target-enforcement"="false" "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'2     ?                                                                                                                                                                      possible intended match
           21: attributes #1 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: attributes #2 = { nounwind }
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           23: 
check:31'0     ~
           24: !llvm.module.flags = !{!0, !1}
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: 
check:31'0     ~
            .
            .
>>>>>>
------------------------------------------
