
In some cases inline asms will contain code that will not work unless the stack is aligned in 
some way, such as calls or SSE instructions on x86, yet will not contain code that does that
alignment within the asm. The compiler should make conservative assumptions about what
the asm might contain and should generate its usual stack alignment code in the prologue if
the ‘alignstack‘ keyword is present.
