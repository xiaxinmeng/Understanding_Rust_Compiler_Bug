 Assembly
idivb   %sil                    ; n % d
movq    -72(%rbp), %r8                   
movb    %ah, (%r8)              ; store the remainder
