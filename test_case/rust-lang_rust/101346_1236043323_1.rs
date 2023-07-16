
%0 = tail call { i32, i32, i32, i32 } asm sideeffect "movl %ebx, ${0:k}\0Acpuid\0Axchgl %ebx, ${0:k}", "=r,={ax},={cx},={dx},1,2,~{memory}"(i32 1, i32 1) #1, !dbg !10, !srcloc !16
