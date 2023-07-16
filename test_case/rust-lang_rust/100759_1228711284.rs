
error: verification with 'FileCheck' failed
status: exit status: 1
command: "/home/gh-fee1-dead/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/home/gh-fee1-dead/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll" "/home/gh-fee1-dead/rust/src/test/codegen/mem-replace-direct-memcpy.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/home/gh-fee1-dead/rust/src/test/codegen/mem-replace-direct-memcpy.rs:20:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memcpy.{{.+}}({{i8\*|ptr}} align 1 %{{.*}}, {{i8\*|ptr}} align 1 %dest, i{{.*}} 1, i1 false)
          ^
/home/gh-fee1-dead/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:47:21: note: scanning from here
; core::mem::replace
                    ^
/home/gh-fee1-dead/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:129:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_4, ptr align 1 %src, i64 1, i1 false)
 ^

Input file: /home/gh-fee1-dead/rust/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll
Check file: /home/gh-fee1-dead/rust/src/test/codegen/mem-replace-direct-memcpy.rs

-dump-input=help explains the following input dump
