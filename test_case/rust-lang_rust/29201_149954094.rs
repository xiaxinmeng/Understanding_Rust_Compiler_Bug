
(lldb) disas
rsnotify-33`notify::sync::mpsc::oneshot::Packet<T>::sent:
    0x100034af0 <+0>:  pushq  %rbp
    0x100034af1 <+1>:  movq   %rsp, %rbp
    0x100034af4 <+4>:  movq   %rdi, -0x10(%rbp)
->  0x100034af8 <+8>:  movq   0x50(%rdi), %rdi
    ...
(lldb) print/x $rdi
(unsigned long) $0 = 0x0000000000001010
