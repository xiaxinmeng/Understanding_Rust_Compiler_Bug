
$ gdb -args rustc -v
GNU gdb (GDB) 7.12.1
(gdb) set environment LIBUNWIND_PRINT_UNWINDING=1 LIBUNWIND_PRINT_APIS=1
(gdb) run
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x763ec2fae54, start_ip=0x763ec2fade0, func=__rust_start_panic, lsda=0x0, personality=0x0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x763ec2ee56d, start_ip=0x763ec2ee560, func=.anonymous., lsda=0x0, personality=0x0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x763ec2ee517, start_ip=0x763ec2ee340, func=_ZN3std9panicking20rust_panic_with_hook17hf2d11de5f905222bE, lsda=0x763ec4a69c8, personality=0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): calling personality function 0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x763600148fb, start_ip=0x763600148e0, func=.anonymous., lsda=0x0, personality=0x0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x763601fefcb, start_ip=0x763601fedd0, func=_ZN5rustc7session11early_error17h0187f9fdb6331d60E, lsda=0x76360521010, personality=0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): calling personality function 0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x7637809902a, start_ip=0x76378096e30, func=_ZN93_$LT$rustc_driver..RustcDefaultCalls$u20$as$u2
0$rustc_driver..CompilerCalls$LT$$u27$a$GT$$GT$8no_input17hc57ca5476db31dacE, lsda=0x76378226ca4, personality=0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): calling personality function 0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x76378094e44, start_ip=0x763780949e0, func=_ZN12rustc_driver12run_compiler17h8fb3410f76685058E, lsda=0x763782268dc, personality=0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): calling personality function 0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x76377fc01b2, start_ip=0x76377fbfe40, func=.anonymous., lsda=0x76378211190, personality=0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): calling personality function 0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): pc=0x763ec2fad9b, start_ip=0x763ec2fad80, func=__rust_maybe_catch_panic, lsda=0x763ec4a6be0, personality=0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): calling personality function 0x763ec2fa8f0
libuwind: unwind_phase1(ex_ojb=0x7642d21b980): _URC_HANDLER_FOUND
libuwind: unwind_phase2(ex_ojb=0x763b3a84880)
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x763ec2fade0, func=__rust_start_panic, sp=0x7f7ffffc8370, lsda=0x0, personality=0x0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x763ec2ee560, func=.anonymous., sp=0x7f7ffffc83c0, lsda=0x0, personality=0x0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x763ec2ee340, func=_ZN3std9panicking20rust_panic_with_hook17hf2d11de5f905222bE, sp=0x7f7ffffc8420, lsda=0x763ec4a69c8, personality=0x763ec2fa8f0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): _URC_CONTINUE_UNWIND
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x76377fc1510, func=.anonymous., sp=0x7f7ffffc84f0, lsda=0x0, personality=0x0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x763780a3a50, func=_ZN12rustc_driver11exit_on_err17h4a90782c564b79d5E, sp=0x7f7ffffc8520, lsda=0x0, personality=0x0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x763780a3be0, func=_ZN12rustc_driver4main17hd2323c6d770bec56E, sp=0x7f7ffffc8560, lsda=0x76378227e24, personality=0x763ec2fa8f0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): _URC_INSTALL_CONTEXT
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): re-entering user code with ip=0x763780a4cab, sp=0x7f7ffffc8560
libuwind: unwind_phase2(ex_ojb=0x763b3a84880)
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x763780a3be0, func=_ZN12rustc_driver4main17hd2323c6d770bec56E, sp=0x7f7ffffc8560, lsda=0x76378227e24, personality=0x763ec2fa8f0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): _URC_CONTINUE_UNWIND
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): start_ip=0x763ec2fad80, func=__rust_maybe_catch_panic, sp=0x7f7ffffc8a80, lsda=0x763ec4a6be0, personality=0x763ec2fa8f0
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): _URC_INSTALL_CONTEXT
libuwind: unwind_phase2(ex_ojb=0x763b3a84880): re-entering user code with ip=0x763ec2fada6, sp=0x7f7ffffc8a80
[Inferior 1 (process 99116) exited with code 0145]
(gdb)
