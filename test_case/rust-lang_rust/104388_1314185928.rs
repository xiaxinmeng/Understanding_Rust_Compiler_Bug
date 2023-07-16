
* thread #10, name = 'risingwave-main', stop reason = EXC_BAD_ACCESS (code=1, address=0x0)
    frame #0: 0x00000001944f6770 libunwind.dylib`libunwind::CFI_Parser<libunwind::LocalAddressSpace>::parseFDEInstructions(libunwind::LocalAddressSpace&, libunwind::CFI_Parser<libunwind::LocalAddressSpace>::FDE_Info const&, libunwind::CFI_Parser<libunwind::LocalAddressSpace>::CIE_Info const&, unsigned long, int, libunwind::CFI_Parser<libunwind::LocalAddressSpace>::PrologInfo*) + 204
libunwind.dylib`libunwind::CFI_Parser<libunwind::LocalAddressSpace>::parseFDEInstructions:
->  0x1944f6770 <+204>: ldrb   w8, [x28], #0x1
    0x1944f6774 <+208>: stur   x28, [x29, #-0x98]
    0x1944f6778 <+212>: cmp    w8, #0x2f
    0x1944f677c <+216>: b.hi   0x1944f72c4               ; <+3104>
Target 0: (risingwave) stopped.
(lldb) bt
* thread #10, name = 'risingwave-main', stop reason = EXC_BAD_ACCESS (code=1, address=0x0)
  * frame #0: 0x00000001944f6770 libunwind.dylib`libunwind::CFI_Parser<libunwind::LocalAddressSpace>::parseFDEInstructions(libunwind::LocalAddressSpace&, libunwind::CFI_Parser<libunwind::LocalAddressSpace>::FDE_Info const&, libunwind::CFI_Parser<libunwind::LocalAddressSpace>::CIE_Info const&, unsigned long, int, libunwind::CFI_Parser<libunwind::LocalAddressSpace>::PrologInfo*) + 204
    frame #1: 0x00000001944f6624 libunwind.dylib`libunwind::UnwindCursor<libunwind::LocalAddressSpace, libunwind::Registers_arm64>::getInfoFromFdeCie(libunwind::CFI_Parser<libunwind::LocalAddressSpace>::FDE_Info const&, libunwind::CFI_Parser<libunwind::LocalAddressSpace>::CIE_Info const&, unsigned long, unsigned long) + 100
    frame #2: 0x00000001944f62fc libunwind.dylib`libunwind::UnwindCursor<libunwind::LocalAddressSpace, libunwind::Registers_arm64>::getInfoFromDwarfSection(unsigned long, libunwind::UnwindInfoSections const&, unsigned int) + 184
    frame #3: 0x00000001944f6220 libunwind.dylib`libunwind::UnwindCursor<libunwind::LocalAddressSpace, libunwind::Registers_arm64>::setInfoBasedOnIPRegister(bool) + 1228
    frame #4: 0x00000001944f86b0 libunwind.dylib`libunwind::UnwindCursor<libunwind::LocalAddressSpace, libunwind::Registers_arm64>::step() + 696
    frame #5: 0x00000001944fb0f0 libunwind.dylib`_Unwind_Backtrace + 348
    frame #6: 0x00000001090822cc risingwave`std::backtrace::Backtrace::create::h908375f7f84cb508 [inlined] std::backtrace_rs::backtrace::libunwind::trace::h471a59e08ff9e5dc at mod.rs:66:5 [opt]
    frame #7: 0x00000001090822bc risingwave`std::backtrace::Backtrace::create::h908375f7f84cb508 [inlined] std::backtrace_rs::backtrace::trace_unsynchronized::h4e694232d85e2708 at mod.rs:66:5 [opt]
    frame #8: 0x00000001090822b0 risingwave`std::backtrace::Backtrace::create::h908375f7f84cb508 at backtrace.rs:333:13 [opt]
    frame #9: 0x00000001056e7b5c risingwave`_$LT$risingwave_meta..error..MetaError$u20$as$u20$core..convert..From$LT$risingwave_meta..error..MetaErrorInner$GT$$GT$::from::hb4b62fbc8685e728(inner=<unavailable>) at error.rs:69:18
    frame #10: 0x0000000105e86adc risingwave`_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h7837bc8fb77e8181(self=<unavailable>) at mod.rs:726:9
    frame #11: 0x00000001056e7fe4 risingwave`risingwave_meta::error::MetaError::permission_denied::h6592a4f64415a283(s=<unavailable>) at error.rs:103:9
