
% lldb /Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-apple-darwin/stage1/bin/rustc -- --crate-name rust_foo src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=66727de05aaedbe2 -C extra-filename=-66727de05aaedbe2 --out-dir /Users/fklock/Dev/Rust/rust-foo/target/debug/deps -L dependency=/Users/fklock/Dev/Rust/rust-foo/target/debug/deps
(lldb) target create "/Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-apple-darwin/stage1/bin/rustc"
Current executable set to '/Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-apple-darwin/stage1/bin/rustc' (x86_64).
(lldb) settings set -- target.run-args  "--crate-name" "rust_foo" "src/main.rs" "--crate-type" "bin" "--emit=dep-info,link" "-C" "debuginfo=2" "-C" "metadata=66727de05aaedbe2" "-C" "extra-filename=-66727de05aaedbe2" "--out-dir" "/Users/fklock/Dev/Rust/rust-foo/target/debug/deps" "-L" "dependency=/Users/fklock/Dev/Rust/rust-foo/target/debug/deps"
(lldb) r
Process 94919 launched: '/Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/build/x86_64-apple-darwin/stage1/bin/rustc' (x86_64)
Process 94919 stopped
* thread #2, stop reason = signal SIGUSR1
    frame #0: 0x0000000100008813 dyld`dyld::fastBindLazySymbol(ImageLoader**, unsigned long)
dyld`dyld::fastBindLazySymbol:
->  0x100008813 <+0>: pushq  %rbp
    0x100008814 <+1>: movq   %rsp, %rbp
    0x100008817 <+4>: pushq  %r14
    0x100008819 <+6>: pushq  %rbx
Target 0: (rustc) stopped.
(lldb) bt
* thread #2, stop reason = signal SIGUSR1
  * frame #0: 0x0000000100008813 dyld`dyld::fastBindLazySymbol(ImageLoader**, unsigned long)
    frame #1: 0x00007fff9b4bf282 libdyld.dylib`dyld_stub_binder + 282
    frame #2: 0x0000000106e4e008 librustc-a48978afc4e78e65.dylib
    frame #3: 0x0000000106abd7cf librustc-a48978afc4e78e65.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::h58c39d6d297406e3 + 399
    frame #4: 0x0000000106ab78f0 librustc-a48978afc4e78e65.dylib`_$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hf4d567cb0f4d1e3c + 80
    frame #5: 0x0000000106ab7b20 librustc-a48978afc4e78e65.dylib`std::panicking::try::do_call::h9b33a9c46ac13d05 + 80
    frame #6: 0x000000010954e658 libstd-e514f1ff308a6a87.dylib`__rust_maybe_catch_panic + 40
    frame #7: 0x0000000106ab7a53 librustc-a48978afc4e78e65.dylib`std::panicking::try::h103437c28968e1f9 + 115
    frame #8: 0x0000000106ab7954 librustc-a48978afc4e78e65.dylib`std::panic::catch_unwind::h36510de003523451 + 84
    frame #9: 0x0000000106abe843 librustc-a48978afc4e78e65.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h9bc74a6deca9ec7d + 195
    frame #10: 0x000000010953fcc1 libstd-e514f1ff308a6a87.dylib`std::sys_common::thread::start_thread::hbccb1a1515a8b445 + 33
    frame #11: 0x0000000109511169 libstd-e514f1ff308a6a87.dylib`std::sys::unix::thread::Thread::new::thread_start::ha9d840ea903629e8 + 9
    frame #12: 0x00007fff9b6dc93b libsystem_pthread.dylib`_pthread_body + 180
    frame #13: 0x00007fff9b6dc887 libsystem_pthread.dylib`_pthread_start + 286
    frame #14: 0x00007fff9b6dc08d libsystem_pthread.dylib`thread_start + 13
(lldb) 
