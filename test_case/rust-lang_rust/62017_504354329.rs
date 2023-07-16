
(lldb) r
Process 17646 launched: '/tmp/foo/t' (x86_64)
Process 17646 stopped
* thread #1, name = 't', stop reason = signal SIGSEGV: invalid address (fault address: 0x36a75)
    frame #0: 0x00000000004022ef t`t::main::h30886ce2ae13d6f9 + 127
t`t::main::h30886ce2ae13d6f9:
->  0x4022ef <+127>: movq   0x36a75, %rsi
    0x4022f7 <+135>: movsd  0x30(%rsp), %xmm0         ; xmm0 = mem[0],zero
    0x4022fd <+141>: movsd  %xmm0, 0x38(%rsp)
    0x402303 <+147>: leaq   0x38(%rsp), %rax
(lldb) bt
* thread #1, name = 't', stop reason = signal SIGSEGV: invalid address (fault address: 0x36a75)
  * frame #0: 0x00000000004022ef t`t::main::h30886ce2ae13d6f9 + 127
    frame #1: 0x0000000000402583 t`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h819e5a92dacb5f12 + 3
    frame #2: 0x000000000040ad13 t`std::panicking::try::do_call::h37d577b51b92ce4b [inlined] std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::hc50706b6e8c189f3 at rt.rs:49:12
    frame #3: 0x000000000040ad07 t`std::panicking::try::do_call::h37d577b51b92ce4b at panicking.rs:294
    frame #4: 0x000000000040c76a t`__rust_maybe_catch_panic at lib.rs:82:7
    frame #5: 0x000000000040b7cd t`std::rt::lang_start_internal::h17a202bf41caec69 [inlined] std::panicking::try::hd2f41f2af573d056 at panicking.rs:273:12
    frame #6: 0x000000000040b78f t`std::rt::lang_start_internal::h17a202bf41caec69 [inlined] std::panic::catch_unwind::h1d1e6c4c3c3f0d9c at panic.rs:388
    frame #7: 0x000000000040b78f t`std::rt::lang_start_internal::h17a202bf41caec69 at rt.rs:48
    frame #8: 0x0000000000402568 t`std::rt::lang_start::h3d4b63434ee976c6 + 56
    frame #9: 0x000000000040240e t`main + 30
    frame #10: 0x00007ffff7d93b6b libc.so.6`__libc_start_main + 235
    frame #11: 0x000000000040216a t`_start + 42
