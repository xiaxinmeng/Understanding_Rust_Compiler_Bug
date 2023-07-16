diff
        movdqa %xmm1,%fs:0xffffffffffffff90
        mov    $something,%eax
        movq   %rax,%xmm1
-       cmpq   $something,%fs:0xffffffffffffff80
+       mov    %fs:0xffffffffffffff80,%rax
        movdqa %xmm1,%fs:0xffffffffffffff80
+       test   %rax,%rax
        je     <_ZN3std9panicking20rust_panic_with_hook17h33ee6001a18cb890E + ofs>
