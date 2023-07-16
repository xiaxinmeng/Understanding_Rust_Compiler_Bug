asm
core::intrinsics::is_nonoverlapping  /home/mark/Build/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-a1a55821ab603792.so [Percent: local period]
 11.47 │      mov    %rdx,%rax                                                                                                                                                                                     ▒
 12.08 │      mov    $0x8,%ecx                                                                                                                                                                                     ▒
  9.55 │      mul    %rcx                                                                                                                                                                                          ▒
 13.37 │    ↓ jo     24                                                                                                                                                                                            ▒
  7.63 │      mov    %rdi,%rcx                                                                                                                                                                                     ▒
       │      sub    %rsi,%rcx                                                                                                                                                                                     ▒
 12.11 │      neg    %rcx                                                                                                                                                                                          ▒
       │      sub    %rsi,%rdi                                                                                                                                                                                     ▒
  1.92 │      cmovbe %rcx,%rdi                                                                                                                                                                                     ▒
 12.74 │      cmp    %rax,%rdi                                                                                                                                                                                     ▒
 10.23 │      setae  %al                                                                                                                                                                                           ▒
  8.91 │    ← retq                                                                                                                                                                                                 ▒
       │24:   ud2                                                                                                                                                                                                  ◆
       │      ud2                                                                                                                                                                                                  ▒
                                                                                                                                                                                                                   ▒
