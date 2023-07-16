
/home/linux1/rust/src/test/assembly/asm/s390x-types.rs:82:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: reg_i32:
                ^
/home/linux1/rust/build/s390x-unknown-linux-gnu/test/assembly/asm/s390x-types.s390x/s390x-types.s:1:2: note: scanning from here
 .text
 ^
/home/linux1/rust/build/s390x-unknown-linux-gnu/test/assembly/asm/s390x-types.s390x/s390x-types.s:23:34: note: possible intended match here
 .section .text._ZN11s390x_types7reg_i3217h8eb3eb08541cc025E,"ax",@progbits


           1:  .text
label:82'0      X~~~~ error: no match found
            2:  .file "s390x_types.63e18542-cgu.0"
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3:  .section .text._ZN11s390x_types9sym_fn_3217h48d94ea6ff47c559E,"ax",@progbits
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4:  .globl _ZN11s390x_types9sym_fn_3217h48d94ea6ff47c559E
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  .p2align 4
label:82'0     ~~~~~~~~~~~
            6:  .type _ZN11s390x_types9sym_fn_3217h48d94ea6ff47c559E,@function
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           18:  br %r14
label:82'0     ~~~~~~~~
           19: .Lfunc_end0:
label:82'0     ~~~~~~~~~~~~
           20:  .size _ZN11s390x_types9sym_fn_3217h48d94ea6ff47c559E, .Lfunc_end0-_ZN11s390x_types9sym_fn_3217h48d94ea6ff47c559E
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21:  .cfi_endproc
label:82'0     ~~~~~~~~~~~~~
           22: 
label:82'0     ~
           23:  .section .text._ZN11s390x_types7reg_i3217h8eb3eb08541cc025E,"ax",@progbits
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:82'1                                      ?                                          possible intended match
           24:  .globl _ZN11s390x_types7reg_i3217h8eb3eb08541cc025E
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  .p2align 4
label:82'0     ~~~~~~~~~~~
           26:  .type _ZN11s390x_types7reg_i3217h8eb3eb08541cc025E,@function
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27: _ZN11s390x_types7reg_i3217h8eb3eb08541cc025E:
label:82'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  .cfi_startproc
label:82'0     ~~~~~~~~~~~~~~~
            .
            .
            .
