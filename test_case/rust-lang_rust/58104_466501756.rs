
0x7fffffffcb40: 0x007d2210      0x00000000      0x00000001      0x00000000
0x7fffffffcb50: 0xffffcff0      0x00000000      0xffffcff0      0x00000000
0x7fffffffcb60: 0xffffcf90      0x00007fff      0x00000000      0x00000000
0x7fffffffcb70: 0x007d0910      0x00000000      0x00000000      0x00000000

   0x00000000004196ca <+90>:    mov    %rdi,0xb8(%rsp)  
   ...
   0x00000000004199c7 <+855>:   mov    0xb8(%rsp),%rcx                                                        
=> 0x00000000004199cf <+863>:   mov    %rax,0x8(%rcx)  

#0  0x00000000004199cf in futures_executor::thread_pool::ThreadPoolBuilder::create (self=0x7fffffffcf90)
    at /home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-executor-0.2.1/src/thread_pool.rs:262

(gdb) x/8x $rcx
0xffffcff0:     Cannot access memory at address 0xffffcff0
