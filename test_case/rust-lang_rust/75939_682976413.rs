
2020-08-28T17:12:34.0362905Z ---- [assembly] assembly/asm/aarch64-types.rs stdout ----
2020-08-28T17:12:34.0363026Z 
2020-08-28T17:12:34.0363334Z error: verification with 'FileCheck' failed
2020-08-28T17:12:34.0363852Z status: exit code: 1
2020-08-28T17:12:34.0364577Z command: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s" "/checkout/obj/build/tmp/distcheck/src/test/assembly/asm/aarch64-types.rs"
2020-08-28T17:12:34.0364890Z stdout:
2020-08-28T17:12:34.0365208Z ------------------------------------------
2020-08-28T17:12:34.0365318Z 
2020-08-28T17:12:34.0365620Z ------------------------------------------
2020-08-28T17:12:34.0365790Z stderr:
2020-08-28T17:12:34.0366097Z ------------------------------------------
2020-08-28T17:12:34.0380310Z /checkout/obj/build/tmp/distcheck/src/test/assembly/asm/aarch64-types.rs:558:17: error: CHECK-LABEL: expected string not found in input
2020-08-28T17:12:34.0380817Z // CHECK-LABEL: issue_75761:
2020-08-28T17:12:34.0380997Z                 ^
2020-08-28T17:12:34.0381479Z /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s:1982:2: note: scanning from here
2020-08-28T17:12:34.0381730Z  .cfi_startproc
2020-08-28T17:12:34.0381901Z  ^
2020-08-28T17:12:34.0382392Z /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s:2003:4: note: possible intended match here
2020-08-28T17:12:34.0382613Z .Lfunc_end72:
2020-08-28T17:12:34.0382768Z    ^
2020-08-28T17:12:34.0382858Z 
2020-08-28T17:12:34.0383288Z Input file: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s
2020-08-28T17:12:34.0384737Z Check file: /checkout/obj/build/tmp/distcheck/src/test/assembly/asm/aarch64-types.rs
2020-08-28T17:12:34.0384881Z 
2020-08-28T17:12:34.0385295Z -dump-input=help explains the following input dump.
2020-08-28T17:12:34.0385413Z 
2020-08-28T17:12:34.0385564Z Input was:
2020-08-28T17:12:34.0385719Z <<<<<<
2020-08-28T17:12:34.0385885Z              .
2020-08-28T17:12:34.0386039Z              .
2020-08-28T17:12:34.0386192Z              .
2020-08-28T17:12:34.0386363Z           1977:  .section .text.v0_f64x2,"ax",@progbits
2020-08-28T17:12:34.0386547Z           1978:  .globl v0_f64x2
2020-08-28T17:12:34.0386732Z           1979:  .p2align 2
2020-08-28T17:12:34.0386906Z           1980:  .type v0_f64x2,@function
2020-08-28T17:12:34.0387080Z           1981: v0_f64x2:
2020-08-28T17:12:34.0387246Z           1982:  .cfi_startproc
2020-08-28T17:12:34.0387594Z label:558'0      X~~~~~~~~~~~~~ error: no match found
2020-08-28T17:12:34.0387944Z           1983:  str x30, [sp, #-32]!
2020-08-28T17:12:34.0388264Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0388443Z           1984:  stp x20, x19, [sp, #16]
2020-08-28T17:12:34.0388769Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0389183Z           1985:  .cfi_def_cfa_offset 32
2020-08-28T17:12:34.0389519Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0389836Z           1986:  .cfi_offset w19, -8
2020-08-28T17:12:34.0390146Z label:558'0     ~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0390466Z           1987:  .cfi_offset w20, -16
2020-08-28T17:12:34.0390792Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0390961Z              .
2020-08-28T17:12:34.0391116Z              .
2020-08-28T17:12:34.0391270Z              .
2020-08-28T17:12:34.0391424Z           1998:  //NO_APP
2020-08-28T17:12:34.0391732Z label:558'0     ~~~~~~~~~
2020-08-28T17:12:34.0391900Z           1999:  str q0, [x20]
2020-08-28T17:12:34.0392204Z label:558'0     ~~~~~~~~~~~~~~
2020-08-28T17:12:34.0392376Z           2000:  ldp x20, x19, [sp, #16]
2020-08-28T17:12:34.0401077Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0401379Z           2001:  ldr x30, [sp], #32
2020-08-28T17:12:34.0401742Z label:558'0     ~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0401925Z           2002:  ret
2020-08-28T17:12:34.0402219Z label:558'0     ~~~~
2020-08-28T17:12:34.0402401Z           2003: .Lfunc_end72:
2020-08-28T17:12:34.0402723Z label:558'0     ~~~~~~~~~~~~~
2020-08-28T17:12:34.0403065Z label:558'1        ?          possible intended match
2020-08-28T17:12:34.0403458Z           2004:  .size v0_f64x2, .Lfunc_end72-v0_f64x2
2020-08-28T17:12:34.0403812Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0404002Z           2005:  .cfi_endproc
2020-08-28T17:12:34.0404300Z label:558'0     ~~~~~~~~~~~~~
2020-08-28T17:12:34.0404465Z           2006: 
2020-08-28T17:12:34.0404746Z label:558'0     ~
2020-08-28T17:12:34.0404914Z           2007:  .type .L__unnamed_1,@object
2020-08-28T17:12:34.0405259Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0405460Z           2008:  .section .rodata..L__unnamed_1,"a",@progbits
2020-08-28T17:12:34.0405828Z label:558'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2020-08-28T17:12:34.0406010Z              .
2020-08-28T17:12:34.0406175Z              .
2020-08-28T17:12:34.0406327Z              .
2020-08-28T17:12:34.0406477Z >>>>>>
2020-08-28T17:12:34.0406566Z 
2020-08-28T17:12:34.0406872Z ------------------------------------------
2020-08-28T17:12:34.0406970Z 
2020-08-28T17:12:34.0407056Z 
2020-08-28T17:12:34.0407140Z 
2020-08-28T17:12:34.0407283Z failures:
2020-08-28T17:12:34.0407593Z     [assembly] assembly/asm/aarch64-types.rs
