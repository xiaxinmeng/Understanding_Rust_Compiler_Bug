
---- [ui] ui/sanitize/issue-72154-lifetime-markers.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/issue-72154-lifetime-markers/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
AddressSanitizer:DEADLYSIGNAL
=================================================================
==426626==ERROR: AddressSanitizer: SEGV on unknown address 0x020080fc12e5 (pc 0x55b3f13d70c2 bp 0x7ffc3f28bb30 sp 0x7ffc3f28b940 T0)
==426626==The signal is caused by a READ memory access.
    #0 0x55b3f13d70c2  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/issue-72154-lifetime-markers/a+0xc70c2)
    #1 0x55b3f13da0a2  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/issue-72154-lifetime-markers/a+0xca0a2)
    #2 0x7fd4ab952e49  (/lib/x86_64-linux-gnu/libc.so.6+0x27e49)
    #3 0x55b3f1317359  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/issue-72154-lifetime-markers/a+0x7359)

AddressSanitizer can not provide additional info.
SUMMARY: AddressSanitizer: SEGV (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/issue-72154-lifetime-markers/a+0xc70c2)
==426626==ABORTING

------------------------------------------


---- [ui] ui/sanitize/address.rs stdout ----

error: error pattern ' AddressSanitizer: stack-buffer-overflow' not found!

error: error pattern ' 'xs' (line 15) <== Memory access at offset' not found!

error: multiple error patterns not found
status: exit status: 1
command: "/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
AddressSanitizer:DEADLYSIGNAL
=================================================================
==426623==ERROR: AddressSanitizer: SEGV on unknown address 0x02008587e89c (pc 0x5564554540b9 bp 0x7ffd621e2800 sp 0x7ffd621e26e0 T0)
==426623==The signal is caused by a READ memory access.
    #0 0x5564554540b9  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xc70b9)
    #1 0x7f8c762e9e49  (/lib/x86_64-linux-gnu/libc.so.6+0x27e49)
    #2 0x556455395349  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0x8349)

AddressSanitizer can not provide additional info.
SUMMARY: AddressSanitizer: SEGV (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xc70b9)
==426623==ABORTING

------------------------------------------


---- [ui] ui/sanitize/use-after-scope.rs stdout ----

error: error pattern ' ERROR: AddressSanitizer: stack-use-after-scope' not found!
status: exit status: 1
command: "/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/use-after-scope/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
AddressSanitizer:DEADLYSIGNAL
=================================================================
==426645==ERROR: AddressSanitizer: SEGV on unknown address 0x0200862997ba (pc 0x55fe7a9b9e69 bp 0x7ffd8a89ef80 sp 0x7ffd8a89ee60 T0)
==426645==The signal is caused by a READ memory access.
    #0 0x55fe7a9b9e69  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/use-after-scope/a+0xc6e69)
    #1 0x7f27523b5e49  (/lib/x86_64-linux-gnu/libc.so.6+0x27e49)
    #2 0x55fe7a8fb349  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/use-after-scope/a+0x8349)

AddressSanitizer can not provide additional info.
SUMMARY: AddressSanitizer: SEGV (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/use-after-scope/a+0xc6e69)
==426645==ABORTING

------------------------------------------


---- [ui] ui/sanitize/new-llvm-pass-manager-thin-lto.rs#opt0 stdout ----

error in revision `opt0`: error pattern ' ERROR: AddressSanitizer: stack-use-after-scope' not found!
status: exit status: 1
command: "/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt0/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
AddressSanitizer:DEADLYSIGNAL
=================================================================
==429414==ERROR: AddressSanitizer: SEGV on unknown address 0x020085494740 (pc 0x55fa31cde252 bp 0x7ffd5275d1f0 sp 0x7ffd5275d000 T0)
==429414==The signal is caused by a READ memory access.
    #0 0x55fa31cde252  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt0/a+0xe5252)
    #1 0x55fa31ce03b2  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt0/a+0xe73b2)
    #2 0x7fbba9271e49  (/lib/x86_64-linux-gnu/libc.so.6+0x27e49)
    #3 0x55fa31c04309  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt0/a+0xb309)

AddressSanitizer can not provide additional info.
SUMMARY: AddressSanitizer: SEGV (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt0/a+0xe5252)
==429414==ABORTING

------------------------------------------


---- [ui] ui/sanitize/new-llvm-pass-manager-thin-lto.rs#opt1 stdout ----

error in revision `opt1`: error pattern ' ERROR: AddressSanitizer: stack-use-after-scope' not found!
status: exit status: 1
command: "/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt1/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
AddressSanitizer:DEADLYSIGNAL
=================================================================
==431968==ERROR: AddressSanitizer: SEGV on unknown address 0x020089056902 (pc 0x560c3cb35bea bp 0x7ffe417e41b0 sp 0x7ffe417e4080 T0)
==431968==The signal is caused by a READ memory access.
    #0 0x560c3cb35bea  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt1/a+0xd5bea)
    #1 0x7f96f619fe49  (/lib/x86_64-linux-gnu/libc.so.6+0x27e49)
    #2 0x560c3ca6d869  (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt1/a+0xd869)

AddressSanitizer can not provide additional info.
SUMMARY: AddressSanitizer: SEGV (/usr/local/google/home/augie/Programming/big/rust/build/x86_64-unknown-linux-gnu/test/ui/sanitize/new-llvm-pass-manager-thin-lto.opt1/a+0xd5bea)
==431968==ABORTING

------------------------------------------
