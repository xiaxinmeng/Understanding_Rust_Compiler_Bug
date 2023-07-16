
                                           ; Basic Block Input Regs: rsp -  Killed Regs: <nothing>
                                           ; 
                                           ; Section __text
                                           ; 
                                           ; Range 0x100000db0 - 0x100000ea1 (241 bytes)
                                           ; File offset 3504 (241 bytes)
                                           ; Flags : 0x80000400
                                           ; 
                                                __ZN3foo18h0e119d36d38e5ceat4v0.0E:        // foo::h0e119d36d38e5ceat::v0.0
    0000000100000db0 65483B242530030000              cmp        rsp, qword [gs:0x330]         ; XREF=0x100000e45
    0000000100000db9 771A                            jnbe       0x100000dd5
                                           ; Basic Block Input Regs: rsp -  Killed Regs: r10 r11
    0000000100000dbb 49BA0F00000000000000            mov        r10, 0xf
    0000000100000dc5 49BB0000000000000000            mov        r11, 0x0
    0000000100000dcf E8C0000000                      call       ___morestack
    0000000100000dd4 C3                              ret        
                                           ; Basic Block Input Regs: rbp -  Killed Regs: rax rsp rbp rdi
    0000000100000dd5 55                              push       rbp                           ; XREF=0x100000db9
    0000000100000dd6 4889E5                          mov        rbp, rsp
    0000000100000dd9 4883EC07                        sub        rsp, 0x7
    0000000100000ddd C60700                          mov        byte [ds:rdi], 0x0
    0000000100000de0 8A45FF                          mov        al, byte [ss:rbp-0x0+var_m1]
    0000000100000de3 884707                          mov        byte [ds:rdi+0x7], al
    0000000100000de6 668B45FD                        mov        ax, word [ss:rbp-0x0+var_m3]
    0000000100000dea 66894705                        mov        word [ds:rdi+0x5], ax
    0000000100000dee 8B45F9                          mov        eax, dword [ss:rbp-0x0+var_m7]
    0000000100000df1 894701                          mov        dword [ds:rdi+0x1], eax
    0000000100000df4 48C7470801000000                mov        qword [ds:rdi+0x8], 0x1
    0000000100000dfc 4883C407                        add        rsp, 0x7
    0000000100000e00 5D                              pop        rbp
    0000000100000e01 C3                              ret        
