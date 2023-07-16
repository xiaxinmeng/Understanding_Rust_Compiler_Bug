
---- [codegen] codegen/issue-69101-bounds-check.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-69101-bounds-check/issue-69101-bounds-check.ll" "/checkout/src/test/codegen/issue-69101-bounds-check.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/src/test/codegen/issue-69101-bounds-check.rs:15:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic_bounds_check
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-69101-bounds-check/issue-69101-bounds-check.ll:141:25: note: found here
; call core::panicking::panic_bounds_check
                        ^~~~~~~~~~~~~~~~~~
/checkout/src/test/codegen/issue-69101-bounds-check.rs:26:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: panic_bounds_check
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-69101-bounds-check/issue-69101-bounds-check.ll:264:25: note: found here
; call core::panicking::panic_bounds_check
                        ^~~~~~~~~~~~~~~~~~

------------------------------------------
