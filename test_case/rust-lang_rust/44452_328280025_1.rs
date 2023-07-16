asm
foo(): # @foo()
  push rax
  mov edi, 4
  ;; -------------------------------------
  ;; what's the point of this memory allocation?
  call operator new(unsigned long)
  mov rdi, rax
  call operator delete(void*)
  ;; -------------------------------------
  mov eax, 314
  pop rcx
  ret
