
$ rustc -v
rustc 0.10-pre (6ca5773 2014-03-13 08:26:40 -0700)
host: x86_64-apple-darwin
$ lldb ./foo
Current executable set to './foo' (x86_64).
(lldb) b rust_fail
Breakpoint 1: where = foo`rust_fail, address = 0x000000010009a1c0
(lldb) r
Process 48220 launched: './foo' (x86_64)
task '<main>' failed at 'booga', foo.rs:2
Process 48220 stopped
* thread #2: tid = 0x37b6fb, 0x000000010009a1c0 foo`rust_fail, stop reason = breakpoint 1.1
    frame #0: 0x000000010009a1c0 foo`rust_fail
foo`rust_fail:
-> 0x10009a1c0:  cmpq   %gs:0x330, %rsp
   0x10009a1c9:  ja     0x10009a1e5               ; rust_fail + 37
   0x10009a1cb:  movabsq $0x68, %r10
   0x10009a1d5:  movabsq $0x0, %r11
(lldb) ^DQuitting LLDB will kill one or more processes. Do you really want to proceed: [Y/n] y
$ lldb -v         
lldb-310.2.36
