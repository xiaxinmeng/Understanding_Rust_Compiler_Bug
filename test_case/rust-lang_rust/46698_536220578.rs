text
Reading symbols from foo...done.
(gdb) break 3
Breakpoint 1 at 0x401650: file .\foo.rs, line 3.
(gdb) r a b
Starting program: C:\Users\steve\tmp\foo.exe a b
[New Thread 8680.0x484c]
[New Thread 8680.0x31fc]
[New Thread 8680.0x38c]
[New Thread 8680.0x20a8]

Breakpoint 1, foo::foo::hf83f4f51e74e413b (x=..., y=...) at .\foo.rs:5
5           x.len() + y.len()
(gdb) p x
$1 = <optimized out>
