llvm
%0 = call i8 asm sideeffect alignstack "move $$t0, $$t0", "=r{$8},r{$8},~{memory}"(i8 %x), !srcloc !2
