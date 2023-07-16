plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMnNINCHblp0
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 12
hw.byteorder: 1234
---
failures:

---- [assembly] src/test/assembly/x86_64-no-jump-tables.rs#unset stdout ----

error in revision `unset`: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-no-jump-tables.unset/x86_64-no-jump-tables.s" "/Users/runner/work/rust/rust/src/test/assembly/x86_64-no-jump-tables.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,unset"
stdout: none
--- stderr -------------------------------
/Users/runner/work/rust/rust/src/test/assembly/x86_64-no-jump-tables.rs:24:12: error: unset: expected string not found in input
 // unset: .LJTI0_0
           ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-no-jump-tables.unset/x86_64-no-jump-tables.s:5:6: note: scanning from here
_foo:
     ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-no-jump-tables.unset/x86_64-no-jump-tables.s:35:24: note: possible intended match here
.set L0_0_set_3, LBB0_3-LJTI0_0

Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/assembly/x86_64-no-jump-tables.unset/x86_64-no-jump-tables.s
Check file: /Users/runner/work/rust/rust/src/test/assembly/x86_64-no-jump-tables.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1:  .section __TEXT,__text,regular,pure_instructions 
            2:  .macosx_version_min 10, 8 
            3:  .globl _foo 
            4:  .p2align 4, 0x90 
            5: _foo: 
check:24'0          X error: no match found
            6:  .cfi_startproc 
check:24'0     ~~~~~~~~~~~~~~~~
            7:  decl %edi 
check:24'0     ~~~~~~~~~~~
            8:  cmpl $4, %edi 
check:24'0     ~~~~~~~~~~~~~~~
            9:  ja LBB0_2 
check:24'0     ~~~~~~~~~~~
           10:  pushq %rbp 
check:24'0     ~~~~~~~~~~~~
            .
            .
            .
           30: LBB0_7: 
check:24'0     ~~~~~~~~
           31:  jmp _bar5 
check:24'0     ~~~~~~~~~~~
           32:  .cfi_endproc 
check:24'0     ~~~~~~~~~~~~~~
           33:  .p2align 2, 0x90 
check:24'0     ~~~~~~~~~~~~~~~~~~
           34:  .data_region jt32 
check:24'0     ~~~~~~~~~~~~~~~~~~~
           35: .set L0_0_set_3, LBB0_3-LJTI0_0 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'1                            ?         possible intended match
           36: .set L0_0_set_4, LBB0_4-LJTI0_0 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: .set L0_0_set_5, LBB0_5-LJTI0_0 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38: .set L0_0_set_6, LBB0_6-LJTI0_0 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39: .set L0_0_set_7, LBB0_7-LJTI0_0 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40: LJTI0_0: 
check:24'0     ~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
