
foo.c:3:22: error: invalid output constraint '{loc}' in asm
  asm ("jmp [$loc]" :"{loc}"(&loc) : :);
                     ^
