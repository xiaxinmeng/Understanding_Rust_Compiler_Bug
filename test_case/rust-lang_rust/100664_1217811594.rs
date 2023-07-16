plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMOF5/Xq9u4N
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
failures:

---- [assembly] src/test/assembly/x86_64-floating-point-clamp.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s" "/Users/runner/work/rust/rust/src/test/assembly/x86_64-floating-point-clamp.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/Users/runner/work/rust/rust/src/test/assembly/x86_64-floating-point-clamp.rs:23:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: ret
                ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:47:2: note: 'next' match was here
 ^
 ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:45:18: note: previous match ended here
 minss xmm0, xmm1
                 ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:46:1: note: non-matching line after previous match is here
 pop rbp

Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s
Check file: /Users/runner/work/rust/rust/src/test/assembly/x86_64-floating-point-clamp.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
        42:  movss xmm1, dword ptr [rip + LCPI1_0] 
        43:  maxss xmm1, xmm0 
        44:  movss xmm0, dword ptr [rip + LCPI1_1] 
        45:  minss xmm0, xmm1 
        46:  pop rbp 
        47:  ret 
next:23      !~~  error: match on wrong line
        48:  .cfi_endproc 
        49:  
        50:  .section __TEXT,__const 
        51: l___unnamed_1: 
        52:  .ascii "assertion failed: min <= max" 
         .
         .
>>>>>>
------------------------------------------
