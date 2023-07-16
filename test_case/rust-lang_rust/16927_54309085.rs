
thread #9: tid = 0x0008, 0x000000010a9164f8 librustc_llvm-4e7c5e5c.dylib`void std::__1::__tree_balance_after_insert<std::__1::__tree_node_base<void*>*>(std::__1::__tree_node_base<void*>*, std::__1::__tree_node_base<void*>*) + 328, stop reason = signal SIGSTOP
    frame #0: 0x000000010a9164f8 librustc_llvm-4e7c5e5c.dylib`void std::__1::__tree_balance_after_insert<std::__1::__tree_node_base<void*>*>(std::__1::__tree_node_base<void*>*, std::__1::__tree_node_base<void*>*) + 328
librustc_llvm-4e7c5e5c.dylib`void std::__1::__tree_balance_after_insert<std::__1::__tree_node_base<void*>*>(std::__1::__tree_node_base<void*>*, std::__1::__tree_node_base<void*>*) + 328:
-> 0x10a9164f8:  movq   (%rcx), %rdx
   0x10a9164fb:  testq  %rdx, %rdx
   0x10a9164fe:  movq   %rdx, 0x8(%rax)
   0x10a916502:  je     0x10a916508               ; void std::__1::__tree_balance_after_insert<std::__1::__tree_node_base<void*>*>(std::__1::__tree_node_base<void*>*, std::__1::__tree_node_base<void*>*) + 344
