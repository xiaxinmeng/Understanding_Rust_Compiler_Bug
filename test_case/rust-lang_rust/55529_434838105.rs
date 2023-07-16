
         asm!("movq $$0, %rax
          movq $$4, %rcx
          1:
          addq $$1, %rax
          cmpq %rax, %rcx
          ja 1b" :::);
    }
