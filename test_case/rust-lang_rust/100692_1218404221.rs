plain
failures:

---- [codegen] src/test/codegen/try_identity.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/try_identity/try_identity.ll" "/checkout/src/test/codegen/try_identity.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/try_identity.rs:12:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: ret i64 %0
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/try_identity/try_identity.ll:8:7: note: scanning from here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/try_identity/try_identity.ll:14:2: note: possible intended match here
 ret i64 %.sroa.07.0.insert.insert

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/try_identity/try_identity.ll
Check file: /checkout/src/test/codegen/try_identity.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'try_identity.55f9fcc1-cgu.0' 
           2: source_filename = "try_identity.55f9fcc1-cgu.0" 
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
           4: target triple = "x86_64-unknown-linux-gnu" 
           5:  
           6: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
           7: define i64 @result_nop_match(i64 %0) unnamed_addr #0 { 
           8: start: 
next:12'0           X error: no match found
           9:  %_2 = and i64 %0, 4294967295 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          10:  %switch = icmp ne i64 %_2, 0 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          11:  %. = zext i1 %switch to i64 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          12:  %.sroa.4.0.extract.shift = and i64 %0, -4294967296 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          13:  %.sroa.07.0.insert.insert = or i64 %.sroa.4.0.extract.shift, %. 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14:  ret i64 %.sroa.07.0.insert.insert 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
next:12'1      ?                                  possible intended match
          15: } 
next:12'0     ~~
          16:  
next:12'0     ~
          17: ; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18: define i64 @result_nop_try_block(i64 %0) unnamed_addr #0 { 
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          19: start: 
next:12'0     ~~~~~~~
           .
           .
>>>>>>
------------------------------------------
