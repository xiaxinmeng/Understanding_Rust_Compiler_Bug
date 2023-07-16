
steve@warmachine:~/tmp$ rustc -g hello.rs
steve@warmachine:~/tmp$ lldb -- ./hello
Current executable set to './hello' (x86_64).
(lldb) b rust_panic
Breakpoint 1: where = hello`rust_panic, address = 0x000000000000ce20
(lldb) run
Process 32608 launching
Process 32608 launched: './hello' (x86_64)
Process 32608 stopped
* thread #1: tid = 32608, 0x00007f703ca0d190, name = 'hello'
    frame #0: 
thread '<main>' panicked at 'booga', hello.rs:2
Process 32608 stopped
* thread #1: tid = 32608, 0x00007f703cc3be20 hello`rust_panic, name = 'hello', stop reason = breakpoint 1.1
    frame #0: 0x00007f703cc3be20 hello`rust_panic
hello`rust_panic:
-> 0x7f703cc3be20:  cmpq   %fs:0x70, %rsp
   0x7f703cc3be29:  ja     0x7f703cc3be45            ; rust_panic + 37
   0x7f703cc3be2b:  movabsq $0xa8, %r10
   0x7f703cc3be35:  movabsq $0x0, %r11
(lldb) version
lldb version 3.5.0 ( revision )
