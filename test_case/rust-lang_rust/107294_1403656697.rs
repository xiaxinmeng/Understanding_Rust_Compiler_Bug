plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7873766cb4a6a0bb00c54496556b38db3fffa5d6)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
i.....i..............i....i..ii.................iii........iii.i........i............... 88/403
....ii.................i............i..i...............i.....i..iiii.......i..i.....i..i 176/403
iiiiii.........i.iii....i..i......................i...ii...i.....ii..i..ii...i.......... 264/403
.....ii........................i.i.ii.i..i...........i..i....i....i..iii...........i...i 352/403
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i..FF..FF.............iiiiiiii.i...................

---- [codegen] tests/codegen/tied-features-strength.rs#DISABLE_NEON stdout ----


error in revision `DISABLE_NEON`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_NEON/tied-features-strength.ll" "/checkout/tests/codegen/tied-features-strength.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,DISABLE_NEON" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/tied-features-strength.rs:13:18: error: DISABLE_NEON: expected string not found in input
// DISABLE_NEON: attributes #0 = { {{.*}} "target-features"="+outline-atomics,-neon,-fp-armv8,+v8a" }
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_NEON/tied-features-strength.ll:1:1: note: scanning from here
; ModuleID = 'tied_features_strength.94bf27d7-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_NEON/tied-features-strength.ll:13:105: note: possible intended match here
attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,-neon,-fp-armv8" }


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_NEON/tied-features-strength.ll
Check file: /checkout/tests/codegen/tied-features-strength.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'tied_features_strength.94bf27d7-cgu.0' 
check:13'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "tied_features_strength.94bf27d7-cgu.0" 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "aarch64-unknown-linux-gnu" 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:13'0     ~
            6: ; tied_features_strength::test 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: define void @_ZN22tied_features_strength4test17hb9a86fea28e79362E() unnamed_addr #0 { 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: start: 
check:13'0     ~~~~~~~
           10:  ret void 
check:13'0     ~~~~~~~~~~
           11: } 
check:13'0     ~~
           12:  
check:13'0     ~
           13: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,-neon,-fp-armv8" } 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:13'1                                                                                                             ?                                                                              possible intended match
           14:  
check:13'0     ~
           15: !llvm.module.flags = !{!0, !1} 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  
check:13'0     ~
           17: !0 = !{i32 7, !"PIC Level", i32 2} 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



---- [codegen] tests/codegen/tied-features-strength.rs#ENABLE_SVE stdout ----

error in revision `ENABLE_SVE`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_SVE/tied-features-strength.ll" "/checkout/tests/codegen/tied-features-strength.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,ENABLE_SVE" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/tied-features-strength.rs:7:16: error: ENABLE_SVE: expected string not found in input
// ENABLE_SVE: attributes #0 = { {{.*}} "target-features"="+outline-atomics,+sve,+neon,+v8a" }
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_SVE/tied-features-strength.ll:1:1: note: scanning from here
; ModuleID = 'tied_features_strength.94bf27d7-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_SVE/tied-features-strength.ll:13:105: note: possible intended match here
attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,+sve,+neon" }


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_SVE/tied-features-strength.ll
Check file: /checkout/tests/codegen/tied-features-strength.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'tied_features_strength.94bf27d7-cgu.0' 
check:7'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "tied_features_strength.94bf27d7-cgu.0" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "aarch64-unknown-linux-gnu" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:7'0     ~
           6: ; tied_features_strength::test 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           8: define void @_ZN22tied_features_strength4test17hb9a86fea28e79362E() unnamed_addr #0 { 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           9: start: 
check:7'0     ~~~~~~~
          10:  ret void 
check:7'0     ~~~~~~~~~~
          11: } 
check:7'0     ~~
          12:  
check:7'0     ~
          13: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,+sve,+neon" } 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:7'1                                                                                                             ?                                                                         possible intended match
          14:  
check:7'0     ~
          15: !llvm.module.flags = !{!0, !1} 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16:  
check:7'0     ~
          17: !0 = !{i32 7, !"PIC Level", i32 2} 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



---- [codegen] tests/codegen/tied-features-strength.rs#DISABLE_SVE stdout ----

error in revision `DISABLE_SVE`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_SVE/tied-features-strength.ll" "/checkout/tests/codegen/tied-features-strength.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,DISABLE_SVE" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/tied-features-strength.rs:10:17: error: DISABLE_SVE: expected string not found in input
// DISABLE_SVE: attributes #0 = { {{.*}} "target-features"="+outline-atomics,-sve,+v8a" }
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_SVE/tied-features-strength.ll:1:1: note: scanning from here
; ModuleID = 'tied_features_strength.94bf27d7-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_SVE/tied-features-strength.ll:13:105: note: possible intended match here
attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,-sve" }


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.DISABLE_SVE/tied-features-strength.ll
Check file: /checkout/tests/codegen/tied-features-strength.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'tied_features_strength.94bf27d7-cgu.0' 
check:10'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "tied_features_strength.94bf27d7-cgu.0" 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "aarch64-unknown-linux-gnu" 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:10'0     ~
            6: ; tied_features_strength::test 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: define void @_ZN22tied_features_strength4test17hb9a86fea28e79362E() unnamed_addr #0 { 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: start: 
check:10'0     ~~~~~~~
           10:  ret void 
check:10'0     ~~~~~~~~~~
           11: } 
check:10'0     ~~
           12:  
check:10'0     ~
           13: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,-sve" } 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:10'1                                                                                                             ?                                                                   possible intended match
           14:  
check:10'0     ~
           15: !llvm.module.flags = !{!0, !1} 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  
check:10'0     ~
           17: !0 = !{i32 7, !"PIC Level", i32 2} 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



---- [codegen] tests/codegen/tied-features-strength.rs#ENABLE_NEON stdout ----

error in revision `ENABLE_NEON`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_NEON/tied-features-strength.ll" "/checkout/tests/codegen/tied-features-strength.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,ENABLE_NEON" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/tied-features-strength.rs:16:17: error: ENABLE_NEON: expected string not found in input
// ENABLE_NEON: attributes #0 = { {{.*}} "target-features"="+outline-atomics,+neon,+fp-armv8,+v8a" }
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_NEON/tied-features-strength.ll:1:1: note: scanning from here
; ModuleID = 'tied_features_strength.94bf27d7-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_NEON/tied-features-strength.ll:13:105: note: possible intended match here
attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,+neon,+fp-armv8" }


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/tied-features-strength.ENABLE_NEON/tied-features-strength.ll
Check file: /checkout/tests/codegen/tied-features-strength.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'tied_features_strength.94bf27d7-cgu.0' 
check:16'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "tied_features_strength.94bf27d7-cgu.0" 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "aarch64-unknown-linux-gnu" 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:16'0     ~
            6: ; tied_features_strength::test 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: define void @_ZN22tied_features_strength4test17hb9a86fea28e79362E() unnamed_addr #0 { 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: start: 
check:16'0     ~~~~~~~
           10:  ret void 
check:16'0     ~~~~~~~~~~
           11: } 
check:16'0     ~~
           12:  
check:16'0     ~
           13: attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "target-cpu"="generic" "target-features"="+outline-atomics,+neon,+fp-armv8" } 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'1                                                                                                             ?                                                                              possible intended match
           14:  
check:16'0     ~
           15: !llvm.module.flags = !{!0, !1} 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  
check:16'0     ~
           17: !0 = !{i32 7, !"PIC Level", i32 2} 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------




failures:
    [codegen] tests/codegen/tied-features-strength.rs#DISABLE_NEON
    [codegen] tests/codegen/tied-features-strength.rs#DISABLE_SVE
    [codegen] tests/codegen/tied-features-strength.rs#ENABLE_NEON
    [codegen] tests/codegen/tied-features-strength.rs#ENABLE_SVE
test result: FAILED. 321 passed; 4 failed; 78 ignored; 0 measured; 0 filtered out; finished in 4.32s

Build completed unsuccessfully in 0:13:03
