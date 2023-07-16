

# *** IR Dump Before Insert XRay ops ***:
# Machine code for function _ZN3std9panicking11begin_panic17h015847775f0133acE: Properties: <Post SSA, tracking liveness, AllVRegsAllocated>
Frame Objects:
  fi#-7: size=4, align=4, fixed, at location [SP-16]
  fi#-6: size=4, align=4, fixed, at location [SP-12]
  fi#-5: size=4, align=4, fixed, at location [SP-8]
  fi#-4: size=4, align=4, fixed, at location [SP-4]
  fi#-3: size=4, align=4, fixed, at location [SP+12]
  fi#-2: size=4, align=4, fixed, at location [SP+8]
  fi#-1: size=4, align=4, fixed, at location [SP+4]
  fi#0: size=16, align=4, at location [SP-32]
  fi#1: size=8, align=4, at location [SP-40]
  fi#2: size=8, align=4, at location [SP-48]
  fi#3: size=8, align=4, at location [SP-56]
  fi#4: size=4, align=4, at location [SP-60]
  fi#5: size=8, align=4, at location [SP-68]
  fi#6: size=4, align=4, at location [SP-72]
  fi#7: size=4, align=4, at location [SP-76]

BB#0: derived from LLVM BB %entry-block
    Live Ins: %EBX %EDI %ESI %EBP
    PUSH32r %EBP<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    %EBP<def> = MOV32rr %ESP; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    PUSH32r %EBX<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    PUSH32r %EDI<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    PUSH32r %ESI<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    %ESP<def,tied1> = SUB32ri8 %ESP<tied0>, 72, %EFLAGS<imp-def,dead>; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    %EAX<def> = MOV32rm %EBP, 1, %noreg, 16, %noreg; mem:LD4[FixedStack-3]
    %ECX<def> = MOV32rm %EBP, 1, %noreg, 12, %noreg; mem:LD4[FixedStack-2]
    %EDX<def> = MOV32rm %EBP, 1, %noreg, 8, %noreg; mem:LD4[FixedStack-1]
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect
    DBG_VALUE %EBP, -56, !"file_line", <!69>; line no:1 indirect
    %ESI<def> = MOV32rr %ESP
    MOV32mr %EBP, 1, %noreg, -28, %noreg, %ESI<kill>; mem:ST4[%6]
    MOV32mi %EBP, 1, %noreg, -16, %noreg, -1; mem:ST4[%7]
    %ESI<def> = LEA32r %EBP, 1, %noreg, -24, %noreg
    MOV32mi %EBP, 1, %noreg, -20, %noreg, <ga:@"__ehhandler$_ZN3std9panicking11begin_panic17h015847775f0133acE">; mem:ST4[%9]
    %EDI<def> = MOV32rm %noreg, 1, %noreg, 0, %FS; mem:LD4[null(addrspace=257)]
    MOV32mr %EBP, 1, %noreg, -24, %noreg, %EDI<kill>; mem:ST4[%11]
    MOV32mr %noreg, 1, %noreg, 0, %FS, %ESI<kill>; mem:ST4[null(addrspace=257)]
    MOV32mr %EBP, 1, %noreg, -52, %noreg, %EDX<kill>; mem:ST4[%12]
    MOV32mr %EBP, 1, %noreg, -48, %noreg, %ECX<kill>; mem:ST4[%13]
    MOV32mr %EBP, 1, %noreg, -56, %noreg, %EAX<kill>; mem:ST4[%arg1]
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -52, %noreg; mem:LD4[%14] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %ECX<def> = MOV32rm %EBP, 1, %noreg, -48, %noreg; mem:LD4[%15] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    MOV32mr %EBP, 1, %noreg, -64, %noreg, %EAX<kill>; mem:ST4[%18] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    MOV32mr %EBP, 1, %noreg, -60, %noreg, %ECX<kill>; mem:ST4[%19] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -56, %noreg; mem:LD4[%arg1] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    MOV32mr %EBP, 1, %noreg, -68, %noreg, %EAX<kill>; mem:ST4[%file_line] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -64, %noreg; mem:LD4[%21] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %ECX<def> = MOV32rm %EBP, 1, %noreg, -60, %noreg; mem:LD4[%22] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %EBP, 1, %noreg, -36, %noreg, %EAX<kill>; mem:ST4[%25](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %EBP, 1, %noreg, -32, %noreg, %ECX; mem:ST4[%26](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -36, %noreg; mem:LD4[%27](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:235 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EBP, 1, %noreg, -44, %noreg, %EAX<kill>; mem:ST4[%31](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:235 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EBP, 1, %noreg, -40, %noreg, %ECX<kill>; mem:ST4[%32](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:235 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mi %EBP, 1, %noreg, -16, %noreg, 0; mem:ST4[%33] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %EAX<def> = MOV32rr %ESP; dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mi %EAX, 1, %noreg, 4, %noreg, 4; mem:ST4[Stack+4] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mi %EAX<kill>, 1, %noreg, 0, %noreg, 8; mem:ST4[Stack] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    CALLpcrel32 <ga:@_ZN5alloc4heap15exchange_malloc17hc245874b0fa682bdE>, <regmask>, %ESP<imp-use>, %ESP<imp-def>, %EAX<imp-def>; dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EBP, 1, %noreg, -72, %noreg, %EAX<kill>; mem:ST4[FixedStack7] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    JMP_1 <BB#1>; dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    Successors according to CFG: BB#1 BB#2

BB#1: derived from LLVM BB %"_ZN5alloc5boxed30_$LT$impl$u20$Box$LT$T$GT$$GT$3new17h3c9ec2d8f3bff52dE.exit"
    Live Ins: %EBP
    Predecessors according to CFG: BB#0
    DBG_VALUE %EBP, -56, !"file_line", <!69>; line no:1 indirect
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -44, %noreg; mem:LD4[%36](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %ECX<def> = MOV32rm %EBP, 1, %noreg, -40, %noreg; mem:LD4[%37](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %EDX<def> = MOV32rm %EBP, 1, %noreg, -72, %noreg; mem:LD4[FixedStack7] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EDX, 1, %noreg, 0, %noreg, %EAX<kill>; mem:ST4[%40] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EDX, 1, %noreg, 4, %noreg, %ECX<kill>; mem:ST4[%41] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -68, %noreg; mem:LD4[%file_line] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %ECX<def> = MOV32rr %ESP; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %ECX, 1, %noreg, 8, %noreg, %EAX<kill>; mem:ST4[Stack+8] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %ECX, 1, %noreg, 0, %noreg, %EDX<kill>; mem:ST4[Stack] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mi %ECX<kill>, 1, %noreg, 4, %noreg, <ga:@vtable6152>; mem:ST4[Stack+4] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    CALLpcrel32 <ga:@_ZN3std9panicking20rust_panic_with_hook17h94399962be666930E>, <regmask>, %ESP<imp-use>, %ESP<imp-def>; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %ESP<def,tied1> = SUB32ri8 %ESP<tied0>, 12, %EFLAGS<imp-def,dead>; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    JMP_1 <BB#3>; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    Successors according to CFG: BB#3 BB#2

BB#3: derived from LLVM BB %unreachable
    Live Ins: %EBP
    Predecessors according to CFG: BB#1
    DBG_VALUE %EBP, -56, !"file_line", <!69>; line no:1 indirect
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect

BB#2: derived from LLVM BB %bb1, EH LANDING PAD
    Live Ins: %EBP
    Predecessors according to CFG: BB#0 BB#1
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect
    PUSH32r %EBP<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    %ESP<def,tied1> = SUB32ri8 %ESP<tied0>, 12, %EFLAGS<imp-def,dead>; flags: FrameSetup dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EBP<def,tied1> = ADD32ri8 %EBP<tied0>, 12, %EFLAGS<imp-def,dead>; flags: FrameSetup
    %ESP<def,tied1> = ADD32ri8 %ESP<tied0>, 12, %EFLAGS<imp-def,dead>; flags: FrameDestroy dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EBP<def> = POP32r %ESP<imp-def>, %ESP<imp-use>; flags: FrameDestroy dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    CLEANUPRET; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376

# End machine code for function _ZN3std9panicking11begin_panic17h015847775f0133acE.

# *** IR Dump Before Implement the 'patchable-function' attribute ***:
# Machine code for function _ZN3std9panicking11begin_panic17h015847775f0133acE: Properties: <Post SSA, tracking liveness, AllVRegsAllocated>
Frame Objects:
  fi#-7: size=4, align=4, fixed, at location [SP-16]
  fi#-6: size=4, align=4, fixed, at location [SP-12]
  fi#-5: size=4, align=4, fixed, at location [SP-8]
  fi#-4: size=4, align=4, fixed, at location [SP-4]
  fi#-3: size=4, align=4, fixed, at location [SP+12]
  fi#-2: size=4, align=4, fixed, at location [SP+8]
  fi#-1: size=4, align=4, fixed, at location [SP+4]
  fi#0: size=16, align=4, at location [SP-32]
  fi#1: size=8, align=4, at location [SP-40]
  fi#2: size=8, align=4, at location [SP-48]
  fi#3: size=8, align=4, at location [SP-56]
  fi#4: size=4, align=4, at location [SP-60]
  fi#5: size=8, align=4, at location [SP-68]
  fi#6: size=4, align=4, at location [SP-72]
  fi#7: size=4, align=4, at location [SP-76]

BB#0: derived from LLVM BB %entry-block
    Live Ins: %EBX %EDI %ESI %EBP
    PUSH32r %EBP<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    %EBP<def> = MOV32rr %ESP; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    PUSH32r %EBX<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    PUSH32r %EDI<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    PUSH32r %ESI<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    %ESP<def,tied1> = SUB32ri8 %ESP<tied0>, 72, %EFLAGS<imp-def,dead>; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    %EAX<def> = MOV32rm %EBP, 1, %noreg, 16, %noreg; mem:LD4[FixedStack-3]
    %ECX<def> = MOV32rm %EBP, 1, %noreg, 12, %noreg; mem:LD4[FixedStack-2]
    %EDX<def> = MOV32rm %EBP, 1, %noreg, 8, %noreg; mem:LD4[FixedStack-1]
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect
    DBG_VALUE %EBP, -56, !"file_line", <!69>; line no:1 indirect
    %ESI<def> = MOV32rr %ESP
    MOV32mr %EBP, 1, %noreg, -28, %noreg, %ESI<kill>; mem:ST4[%6]
    MOV32mi %EBP, 1, %noreg, -16, %noreg, -1; mem:ST4[%7]
    %ESI<def> = LEA32r %EBP, 1, %noreg, -24, %noreg
    MOV32mi %EBP, 1, %noreg, -20, %noreg, <ga:@"__ehhandler$_ZN3std9panicking11begin_panic17h015847775f0133acE">; mem:ST4[%9]
    %EDI<def> = MOV32rm %noreg, 1, %noreg, 0, %FS; mem:LD4[null(addrspace=257)]
    MOV32mr %EBP, 1, %noreg, -24, %noreg, %EDI<kill>; mem:ST4[%11]
    MOV32mr %noreg, 1, %noreg, 0, %FS, %ESI<kill>; mem:ST4[null(addrspace=257)]
    MOV32mr %EBP, 1, %noreg, -52, %noreg, %EDX<kill>; mem:ST4[%12]
    MOV32mr %EBP, 1, %noreg, -48, %noreg, %ECX<kill>; mem:ST4[%13]
    MOV32mr %EBP, 1, %noreg, -56, %noreg, %EAX<kill>; mem:ST4[%arg1]
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -52, %noreg; mem:LD4[%14] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %ECX<def> = MOV32rm %EBP, 1, %noreg, -48, %noreg; mem:LD4[%15] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    MOV32mr %EBP, 1, %noreg, -64, %noreg, %EAX<kill>; mem:ST4[%18] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    MOV32mr %EBP, 1, %noreg, -60, %noreg, %ECX<kill>; mem:ST4[%19] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -56, %noreg; mem:LD4[%arg1] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    MOV32mr %EBP, 1, %noreg, -68, %noreg, %EAX<kill>; mem:ST4[%file_line] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -64, %noreg; mem:LD4[%21] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %ECX<def> = MOV32rm %EBP, 1, %noreg, -60, %noreg; mem:LD4[%22] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %EBP, 1, %noreg, -36, %noreg, %EAX<kill>; mem:ST4[%25](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %EBP, 1, %noreg, -32, %noreg, %ECX; mem:ST4[%26](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -36, %noreg; mem:LD4[%27](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:235 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EBP, 1, %noreg, -44, %noreg, %EAX<kill>; mem:ST4[%31](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:235 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EBP, 1, %noreg, -40, %noreg, %ECX<kill>; mem:ST4[%32](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:235 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mi %EBP, 1, %noreg, -16, %noreg, 0; mem:ST4[%33] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %EAX<def> = MOV32rr %ESP; dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mi %EAX, 1, %noreg, 4, %noreg, 4; mem:ST4[Stack+4] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mi %EAX<kill>, 1, %noreg, 0, %noreg, 8; mem:ST4[Stack] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    CALLpcrel32 <ga:@_ZN5alloc4heap15exchange_malloc17hc245874b0fa682bdE>, <regmask>, %ESP<imp-use>, %ESP<imp-def>, %EAX<imp-def>; dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EBP, 1, %noreg, -72, %noreg, %EAX<kill>; mem:ST4[FixedStack7] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    JMP_1 <BB#1>; dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    Successors according to CFG: BB#1 BB#2

BB#1: derived from LLVM BB %"_ZN5alloc5boxed30_$LT$impl$u20$Box$LT$T$GT$$GT$3new17h3c9ec2d8f3bff52dE.exit"
    Live Ins: %EBP
    Predecessors according to CFG: BB#0
    DBG_VALUE %EBP, -56, !"file_line", <!69>; line no:1 indirect
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -44, %noreg; mem:LD4[%36](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %ECX<def> = MOV32rm %EBP, 1, %noreg, -40, %noreg; mem:LD4[%37](noalias=!106) dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %EDX<def> = MOV32rm %EBP, 1, %noreg, -72, %noreg; mem:LD4[FixedStack7] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EDX, 1, %noreg, 0, %noreg, %EAX<kill>; mem:ST4[%40] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    MOV32mr %EDX, 1, %noreg, 4, %noreg, %ECX<kill>; mem:ST4[%41] dbg:/opt/rust/build-debug-assertions/t/../../src/liballoc/boxed.rs:236 @[ /opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384 ]
    %EAX<def> = MOV32rm %EBP, 1, %noreg, -68, %noreg; mem:LD4[%file_line] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %ECX<def> = MOV32rr %ESP; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %ECX, 1, %noreg, 8, %noreg, %EAX<kill>; mem:ST4[Stack+8] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mr %ECX, 1, %noreg, 0, %noreg, %EDX<kill>; mem:ST4[Stack] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    MOV32mi %ECX<kill>, 1, %noreg, 4, %noreg, <ga:@vtable6152>; mem:ST4[Stack+4] dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    CALLpcrel32 <ga:@_ZN3std9panicking20rust_panic_with_hook17h94399962be666930E>, <regmask>, %ESP<imp-use>, %ESP<imp-def>; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    %ESP<def,tied1> = SUB32ri8 %ESP<tied0>, 12, %EFLAGS<imp-def,dead>; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    JMP_1 <BB#3>; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:384
    Successors according to CFG: BB#3 BB#2

BB#3: derived from LLVM BB %unreachable
    Live Ins: %EBP
    Predecessors according to CFG: BB#1
    DBG_VALUE %EBP, -56, !"file_line", <!69>; line no:1 indirect
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect

BB#2: derived from LLVM BB %bb1, EH LANDING PAD
    Live Ins: %EBP
    Predecessors according to CFG: BB#0 BB#1
    DBG_VALUE %EBP, -52, !"msg", <!69>; line no:1 indirect
    PUSH32r %EBP<kill>, %ESP<imp-def>, %ESP<imp-use>; flags: FrameSetup
    CFI_INSTRUCTION <call frame instruction>
    CFI_INSTRUCTION <call frame instruction>
    %ESP<def,tied1> = SUB32ri8 %ESP<tied0>, 12, %EFLAGS<imp-def,dead>; flags: FrameSetup dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EBP<def,tied1> = ADD32ri8 %EBP<tied0>, 12, %EFLAGS<imp-def,dead>; flags: FrameSetup
    %ESP<def,tied1> = ADD32ri8 %ESP<tied0>, 12, %EFLAGS<imp-def,dead>; flags: FrameDestroy dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    %EBP<def> = POP32r %ESP<imp-def>, %ESP<imp-use>; flags: FrameDestroy dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376
    CLEANUPRET; dbg:/opt/rust/build-debug-assertions/t/../../src/libstd/panicking.rs:376

# End machine code for function _ZN3std9panicking11begin_panic17h015847775f0133acE.

rustc: /opt/rust/src/llvm/lib/CodeGen/AsmPrinter/CodeViewDebug.cpp:202: llvm::codeview::TypeIndex llvm::CodeViewDebug::getScopeIndex(const llvm::DIScope*): Assertion `!isa<DIType>(Scope) && "shouldn't make a namespace scope for a type"' failed.
