
0x0000000100000da6 <_rust_main+54>:    lea    rdi,[rbp-0x70]
0x0000000100000daa <_rust_main+58>:    lea    r14,[rip+0x12cf]        # 0x100002080 <tydesc_1980>
0x0000000100000db1 <_rust_main+65>:    mov    rdx,r14
0x0000000100000db4 <_rust_main+68>:    mov    ecx,0x90
0x0000000100000db9 <_rust_main+73>:    call   0x100001c04 <dyld_stub__ZN2rt18rt_exchange_malloc16_63f842b316661113_06E> ; allocate
0x0000000100000dbe <_rust_main+78>:    mov    rbx,QWORD PTR [rbp-0x70] ; rbx = result of malloc
... straight line code ...
0x0000000100000ddd <_rust_main+109>:    mov    QWORD PTR [rbx+0x30],0x0 ; store None tag
... straight line code ...
0x0000000100000dfa <_rust_main+138>:    lea    rbx,[rbx+0x20] ; skip over allocation header
... straight line code ...
0x0000000100000e66 <_rust_main+246>:    mov    rax,QWORD PTR [rbx+0x10] ; rax = None tag
0x0000000100000e6a <_rust_main+250>:    mov    QWORD PTR [rbp-0xa0],rax ; copy None tag to new variant
0x0000000100000e71 <_rust_main+257>:    mov    QWORD PTR [rbp-0x98],rbx
0x0000000100000e78 <_rust_main+264>:    mov    rbx,QWORD PTR [rbx+0x18] ; rbx = eagerly load value of the ~str (which is undefined since the tag discriminant is None)
0x0000000100000e7c <_rust_main+268>:    cmp    rax,0x1 ; was tag Some?
0x0000000100000e80 <_rust_main+272>:    jne    0x100000ebe ; taken <_rust_main+334> ;
...
0x0000000100000ebe <_rust_main+334>:    mov    r12,rbx ; r12 = undefined value of ~str
... straight line code ...
0x0000000100000f23 <_rust_main+435>:    cmp    QWORD PTR [rbp-0xa0],0x1 ; is tag Some?
0x0000000100000f2b <_rust_main+443>:    setne  al ; invert previous check; al is now 1 iff tag is *None* (i.e. al is always 1)
0x0000000100000f2e <_rust_main+446>:    test   r12,r12 ; is tag None?
0x0000000100000f31 <_rust_main+449>:    je     0x100000f3f <_rust_main+463> ; ***valgrind error***; conditional branch depends on undefined value. Here, we are checking to see whether the pointer is NULL.
0x0000000100000f33 <_rust_main+451>:    test   al,al ; is tag None?
0x0000000100000f35 <_rust_main+453>:    jne    0x100000f3f <_rust_main+463> ; taken
0x0000000100000f37 <_rust_main+455>:    mov    rdx,r12
0x0000000100000f3a <_rust_main+458>:    call   0x100001bfe <dyld_stub__ZN2rt16rt_exchange_free17_bea361ae477736593_06E> ; free the allocation
0x0000000100000f3f <_rust_main+463>:    cmp    QWORD PTR [rbp-0x90],0x1
