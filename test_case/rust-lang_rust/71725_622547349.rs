
pub fn f(x: usize) -> Option<i32> {
    const T: [i32; 6] = [4, 8, 15, 16, 23, 42];
    T.get(x).copied()
}

pub fn f2(x: usize) -> Option<i32> {
    [4, 8, 15, 16, 23, 42].get(x).copied()
}


example::f:
        xor     eax, eax
        cmp     rdi, 5
        ja      .LBB0_1
        lea     rcx, [rip + .L__unnamed_1]
        lea     rcx, [rcx + 4*rdi]
        test    rcx, rcx
        je      .LBB0_4
        mov     edx, dword ptr [rcx]
        mov     eax, 1
.LBB0_4:
        ret
.LBB0_1:
        ret
