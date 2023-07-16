plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMA5jilobsBU
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
failures:

---- [assembly] tests/assembly/slice-is_ascii.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/slice-is_ascii/slice-is_ascii.s" "/Users/runner/work/rust/rust/tests/assembly/slice-is_ascii.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/Users/runner/work/rust/rust/tests/assembly/slice-is_ascii.rs:20:12: error: CHECK: expected string not found in input
/Users/runner/work/rust/rust/tests/assembly/slice-is_ascii.rs:20:12: error: CHECK: expected string not found in input
 // CHECK: .[[LOOPHEAD:.+]]:
           ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/slice-is_ascii/slice-is_ascii.s:6:23: note: scanning from here
_is_ascii_simple_demo:
                      ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/slice-is_ascii/slice-is_ascii.s:14:1: note: possible intended match here
LBB0_1:

Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/slice-is_ascii/slice-is_ascii.s
Check file: /Users/runner/work/rust/rust/tests/assembly/slice-is_ascii.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1:  .section __TEXT,__text,regular,pure_instructions 
            2:  .macosx_version_min 10, 8 
            3:  .intel_syntax noprefix 
            4:  .globl _is_ascii_simple_demo 
            5:  .p2align 4, 0x90 
            6: _is_ascii_simple_demo: 
check:20'0                           X error: no match found
            7:  .cfi_startproc 
check:20'0     ~~~~~~~~~~~~~~~~
            8:  push rbp 
check:20'0     ~~~~~~~~~~
            9:  .cfi_def_cfa_offset 16 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           10:  .cfi_offset rbp, -16 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~
           11:  mov rbp, rsp 
check:20'0     ~~~~~~~~~~~~~~
           12:  .cfi_def_cfa_register rbp 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  .p2align 4, 0x90 
check:20'0     ~~~~~~~~~~~~~~~~~~
           14: LBB0_1: 
check:20'0     ~~~~~~~~
check:20'1     ?        possible intended match
           15:  mov rax, rsi 
check:20'0     ~~~~~~~~~~~~~~
           16:  sub rsi, 1 
check:20'0     ~~~~~~~~~~~~
           17:  jb LBB0_3 
check:20'0     ~~~~~~~~~~~
           18:  cmp byte ptr [rdi + rax - 1], 0 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  jns LBB0_1 
check:20'0     ~~~~~~~~~~~~
           20: LBB0_3: 
check:20'0     ~~~~~~~~
           21:  test rax, rax 
check:20'0     ~~~~~~~~~~~~~~~
           22:  sete al 
check:20'0     ~~~~~~~~~
           23:  pop rbp 
check:20'0     ~~~~~~~~~
           24:  ret 
check:20'0     ~~~~~
           25:  .cfi_endproc 
check:20'0     ~~~~~~~~~~~~~~
           26:  
check:20'0     ~
           27: .subsections_via_symbols 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



