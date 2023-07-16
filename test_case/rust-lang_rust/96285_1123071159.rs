plain
Some tests failed in compiletest suite=codegen mode=codegen host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu

failures:

---- [codegen] src/test/codegen/virtual-function-elimination.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll" "/checkout/src/test/codegen/virtual-function-elimination.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/virtual-function-elimination.rs:66:12: error: CHECK: expected string not found in input
 // CHECK: @llvm.type.checked.load({{.*}}, i32 24, metadata !"[[MANGLED_TYPE0:[0-9a-zA-Z_]+]]")
           ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll:70:88: note: scanning from here
; <virtual_function_elimination::S as virtual_function_elimination::V>::public_function
                                                                                       ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll:82:29: note: possible intended match here
 %1 = tail call { i8*, i1 } @llvm.type.checked.load(i8* nonnull %0, i32 12, metadata !"NtCsfRpWlKdQPZn_28virtual_function_elimination1V")


Input file: /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll
Check file: /checkout/src/test/codegen/virtual-function-elimination.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           65: define internal i32 @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1U13subtrait_usedB4_(%S* noalias nocapture noundef nonnull readonly align 1 %self) unnamed_addr #1 { 
           67:  ret i32 4 
           68: } 
           69:  
           69:  
           70: ; <virtual_function_elimination::S as virtual_function_elimination::V>::public_function 
check:66'0                                                                                            X error: no match found
           71: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72: define i32 @_RNvXs0_CsfRpWlKdQPZn_28virtual_function_eliminationNtB5_1SNtB5_1V15public_function(%S* noalias nocapture nonnull readonly align 1 %self) unnamed_addr #1 { 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73: start: 
check:66'0     ~~~~~~~
           74:  ret i32 6 
check:66'0     ~~~~~~~~~~~
           75: } 
check:66'0     ~~
           76:  
check:66'0     ~
           77: ; virtual_function_elimination::taking_v 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           78: ; Function Attrs: nonlazybind uwtable 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79: define i32 @_RNvCsfRpWlKdQPZn_28virtual_function_elimination8taking_v({}* noundef nonnull align 1 %v.0, [3 x i32]* noalias noundef readonly align 4 dereferenceable(12) %v.1) unnamed_addr #2 { 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80: start: 
check:66'0     ~~~~~~~
           81:  %0 = bitcast [3 x i32]* %v.1 to i8* 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82:  %1 = tail call { i8*, i1 } @llvm.type.checked.load(i8* nonnull %0, i32 12, metadata !"NtCsfRpWlKdQPZn_28virtual_function_elimination1V") 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:66'1                                 ?                                                                                                              possible intended match
           83:  %2 = extractvalue { i8*, i1 } %1, 0 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84:  %3 = bitcast i8* %2 to i32 ({}*)* 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           85:  %4 = tail call i32 %3({}* noundef nonnull align 1 %v.0) 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           86:  ret i32 %4 
check:66'0     ~~~~~~~~~~~~
           87: } 
check:66'0     ~~
            .
            .
>>>>>>
------------------------------------------
