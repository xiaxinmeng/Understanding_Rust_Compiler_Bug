diff
        movdqa %xmm1,%fs:0xffffffffffffff70
        mov    $something,%eax
        movq   %rax,%xmm1
-       mov    %fs:0xffffffffffffff60,%rax
+       cmpq   $something,%fs:0xffffffffffffff60
        movdqa %xmm1,%fs:0xffffffffffffff60
-       test   %rax,%rax
        je     <_ZN3std9panicking20rust_panic_with_hook17h40c40e76fede9cc3E + ofs>
        movq   %xmm0,%rbx
        test   %rbx,%rbx
