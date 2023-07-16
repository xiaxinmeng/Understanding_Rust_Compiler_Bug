
   0x56673150 <+0>: push   %ebx
   0x56673151 <+1>: push   %esi                                ; !!! -6399848i32 gets pushed, into address 0xff9e5824
   0x56673152 <+2>: push   %eax
   0x56673153 <+3>: mov    0x10(%esp),%esi
   0x56673157 <+7>: call   0x5667315c <_ZN9term_size10dimensions17h0c2d81d1b27cee8bE+12>
   0x5667315c <+12>:    pop    %ebx
   0x5667315d <+13>:    movl   $0x0,(%esp)
   0x56673164 <+20>:    add    $0xd9ea4,%ebx
   0x5667316a <+26>:    sub    $0x4,%esp
   0x5667316d <+29>:    lea    0x4(%esp),%eax                     ; somewhat suspicious, puts 0xff9e5820 into %eax (just 4 bytes away from the place where our %esi is)
   0x56673171 <+33>:    push   %eax
   0x56673172 <+34>:    push   $0x5413
   0x56673177 <+39>:    push   $0x1
   0x56673179 <+41>:    call   0x565c15d0 <ioctl@plt>             ; !!! stack location (0xff9e5824) overwritten somewhere inside this function?!
   0x5667317e <+46>:    add    $0x10,%esp
   0x56673181 <+49>:    cmp    $0xffffffff,%eax
   0x56673184 <+52>:    je     0x566731a5 <_ZN9term_size10dimensions17h0c2d81d1b27cee8bE+85>
   0x56673186 <+54>:    mov    (%esp),%ecx
   0x56673189 <+57>:    mov    %ecx,%eax
   0x5667318b <+59>:    and    $0xffff,%eax
   0x56673190 <+64>:    je     0x566731a5 <_ZN9term_size10dimensions17h0c2d81d1b27cee8bE+85>
   0x56673192 <+66>:    shr    $0x10,%ecx
   0x56673195 <+69>:    je     0x566731a5 <_ZN9term_size10dimensions17h0c2d81d1b27cee8bE+85>
   0x56673197 <+71>:    movl   $0x1,(%esi)
   0x5667319d <+77>:    mov    %ecx,0x4(%esi)
   0x566731a0 <+80>:    mov    %eax,0x8(%esi)
   0x566731a3 <+83>:    jmp    0x566731ab <_ZN9term_size10dimensions17h0c2d81d1b27cee8bE+91>
   0x566731a5 <+85>:    movl   $0x0,(%esi)
   0x566731ab <+91>:    mov    %esi,%eax
   0x566731ad <+93>:    add    $0x4,%esp
   0x566731b0 <+96>:    pop    %esi                             ; !!! 0 gets popped, from address 0xff9e5824
   0x566731b1 <+97>:    pop    %ebx
   0x566731b2 <+98>:    ret    $0x4
