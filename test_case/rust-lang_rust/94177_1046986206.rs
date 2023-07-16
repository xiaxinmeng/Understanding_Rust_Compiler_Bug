plain
failures:

---- [codegen] codegen/try_identity.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/try_identity/try_identity.ll" "/checkout/src/test/codegen/try_identity.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/try_identity.rs:17:15: error: CHECK-NOT: excluded string found in input
// CHECK-NOT: br {{.*}}
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/try_identity/try_identity.ll:37:2: note: found here
 br label %bb4

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/try_identity/try_identity.ll
Check file: /checkout/src/test/codegen/try_identity.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       32: 
       33: bb2: ; preds = %start
       34:  unreachable
       35: 
       36: bb1: ; preds = %start
       37:  br label %bb4
not:17      !~~~~~~~~~~~~ error: no match expected
       38: 
       39: bb3: ; preds = %start
       40:  br label %bb4
       41: 
       42: bb4: ; preds = %bb1, %bb3
        .
        .
>>>>>>

