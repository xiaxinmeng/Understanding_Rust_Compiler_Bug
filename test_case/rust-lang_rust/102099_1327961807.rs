plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] src/test/codegen/unwind-landingpad-inline.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-inline/unwind-landingpad-inline.ll" "/checkout/src/test/codegen/unwind-landingpad-inline.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/unwind-landingpad-inline.rs:31:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: call void %g()
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-inline/unwind-landingpad-inline.ll:44:7: note: scanning from here
      ^
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-inline/unwind-landingpad-inline.ll:45:4: note: possible intended match here
 invoke void %g()

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-inline/unwind-landingpad-inline.ll
Check file: /checkout/src/test/codegen/unwind-landingpad-inline.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          39:  ret void 
          40: } 
          41:  
          42: ; Function Attrs: nonlazybind uwtable 
          43: define void @check_eliminate_noop_drop(ptr nocapture noundef nonnull readonly %g) unnamed_addr #0 personality ptr @rust_eh_personality { 
          44: start: 
next:31'0           X error: no match found
          45:  invoke void %g() 
next:31'0     ~~~~~~~~~~~~~~~~~~
next:31'1        ?               possible intended match
          46:  to label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h5911989c448e8dabE.exit" unwind label %cleanup 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          47:  
next:31'0     ~
          48: cleanup: ; preds = %start 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          49:  %0 = landingpad { ptr, i32 } 
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          50:  cleanup 
next:31'0     ~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
