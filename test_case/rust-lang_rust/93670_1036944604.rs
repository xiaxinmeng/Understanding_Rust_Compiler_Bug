plain
failures:

---- [codegen] codegen/fastcall-inreg.rs stdout ----

Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll" "/checkout/src/test/codegen/fastcall-inreg.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/fastcall-inreg.rs:73:12: error: CHECK: expected string not found in input
 // CHECK: @f6(i1 inreg zeroext %_1, i32 inreg %_2, i32 %_3)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll:31:49: note: scanning from here
define x86_fastcallcc void @f5(i64 %_1, i32 %_2) unnamed_addr #0 {
                                                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll:37:36: note: possible intended match here
define x86_fastcallcc void @f6(i1 inreg noundef zeroext %_1, i32 inreg %_2, i32 %_3) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           26: start: 
           27:  ret void 
           28: } 
           29:  
           30: ; Function Attrs: nonlazybind uwtable 
           31: define x86_fastcallcc void @f5(i64 %_1, i32 %_2) unnamed_addr #0 { 
check:73'0                                                     X~~~~~~~~~~~~~~~~~~ error: no match found
           32: start: 
check:73'0     ~~~~~~~
           33:  ret void 
check:73'0     ~~~~~~~~~~
           34: } 
check:73'0     ~~
           35:  
check:73'0     ~
           36: ; Function Attrs: nonlazybind uwtable 
check:73'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: define x86_fastcallcc void @f6(i1 inreg noundef zeroext %_1, i32 inreg %_2, i32 %_3) unnamed_addr #0 { 
check:73'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:73'1                                        ?                                                                    possible intended match
           38: start: 
check:73'0     ~~~~~~~
           39:  ret void 
check:73'0     ~~~~~~~~~~
           40: } 
check:73'0     ~~
           41:  
check:73'0     ~
           42: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium" } 
check:73'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

