plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] src/test/codegen/zst-offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll" "/checkout/src/test/codegen/zst-offset.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/zst-offset.rs:16:11: error: CHECK: expected string not found in input
// CHECK: getelementptr i8, {{.+}}, [[USIZE]] 8
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:13:27: note: scanning from here
define void @scalar_layout(i64* noalias noundef readonly align 8 dereferenceable(8) %s) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:13:27: note: with "USIZE" equal to "i64"
define void @scalar_layout(i64* noalias noundef readonly align 8 dereferenceable(8) %s) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:13:66: note: possible intended match here
define void @scalar_layout(i64* noalias noundef readonly align 8 dereferenceable(8) %s) unnamed_addr #0 {
/checkout/src/test/codegen/zst-offset.rs:25:11: error: CHECK: expected string not found in input
/checkout/src/test/codegen/zst-offset.rs:25:11: error: CHECK: expected string not found in input
// CHECK: getelementptr i8, {{.+}}, [[USIZE]] 12
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:19:31: note: scanning from here
define void @scalarpair_layout({ i64, i32 }* noalias noundef readonly align 8 dereferenceable(16) %s) unnamed_addr #0 {
                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:19:31: note: with "USIZE" equal to "i64"
define void @scalarpair_layout({ i64, i32 }* noalias noundef readonly align 8 dereferenceable(16) %s) unnamed_addr #0 {
                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:19:79: note: possible intended match here
define void @scalarpair_layout({ i64, i32 }* noalias noundef readonly align 8 dereferenceable(16) %s) unnamed_addr #0 {
/checkout/src/test/codegen/zst-offset.rs:37:11: error: CHECK: expected string not found in input
/checkout/src/test/codegen/zst-offset.rs:37:11: error: CHECK: expected string not found in input
// CHECK: getelementptr i8, {{.+}}, [[USIZE]] 32
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:25:27: note: scanning from here
define void @vector_layout(<4 x i64>* noalias noundef readonly align 32 dereferenceable(32) %s) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:25:27: note: with "USIZE" equal to "i64"
define void @vector_layout(<4 x i64>* noalias noundef readonly align 32 dereferenceable(32) %s) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll:25:73: note: possible intended match here
define void @vector_layout(<4 x i64>* noalias noundef readonly align 32 dereferenceable(32) %s) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/zst-offset/zst-offset.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            8: start: 
            9:  ret void 
           10: } 
           11:  
           12: ; Function Attrs: nonlazybind uwtable 
           13: define void @scalar_layout(i64* noalias noundef readonly align 8 dereferenceable(8) %s) unnamed_addr #0 { 
check:16'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:16'1                                                                                                                with "USIZE" equal to "i64"
check:16'2                                                                      ?                                         possible intended match
           14: start: 
check:16'0     ~~~~~~~
           15:  ret void 
check:16'0     ~~~~~~~~~~
           16: } 
check:16'0     ~~
           17:  
check:16'0     ~
           18: ; Function Attrs: nonlazybind uwtable 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19: define void @scalarpair_layout({ i64, i32 }* noalias noundef readonly align 8 dereferenceable(16) %s) unnamed_addr #0 { 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:25'0                                   X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:25'1                                                                                                                              with "USIZE" equal to "i64"
check:25'2                                                                                   ?                                          possible intended match
           20: start: 
check:25'0     ~~~~~~~
           21:  ret void 
check:25'0     ~~~~~~~~~~
           22: } 
check:25'0     ~~
           23:  
check:25'0     ~
           24: ; Function Attrs: nonlazybind uwtable 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: define void @vector_layout(<4 x i64>* noalias noundef readonly align 32 dereferenceable(32) %s) unnamed_addr #0 { 
check:25'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:37'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:37'1                                                                                                                        with "USIZE" equal to "i64"
check:37'2                                                                             ?                                          possible intended match
           26: start: 
check:37'0     ~~~~~~~
           27:  ret void 
check:37'0     ~~~~~~~~~~
           28: } 
check:37'0     ~~
           29:  
check:37'0     ~
           30: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
