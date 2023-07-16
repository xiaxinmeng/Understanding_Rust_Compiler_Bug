
.text:00000001800012C4                 mov     rax, [rcx]
.text:00000001800012C7                 lea     rdx, [rax+8]
.text:00000001800012CB                 cmp     rdx, [rcx+8]
.text:00000001800012CF                 jnb     short loc_1800012E0
.text:00000001800012D1                 mov     [rcx], rdx
.text:00000001800012D4
.text:00000001800012D4 loc_1800012D4:                          ; CODE XREF: alloctest+2A↓j
...
.text:00000001800012E0 ; ---------------------------------------------------------------------------
.text:00000001800012E0
.text:00000001800012E0 loc_1800012E0:                          ; CODE XREF: alloctest+F↑j
.text:00000001800012E0                 mov     edx, 8
.text:00000001800012E5                 call    _ZN5arena13DroplessArena18grow_and_alloc_raw17ha43c166c6d43e92fE ; arena::DroplessArena::grow_and_alloc_raw::ha43c166c6d43e92f
.text:00000001800012EA                 jmp     short loc_1800012D4
