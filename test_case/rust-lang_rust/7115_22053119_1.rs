 Assembly
idivb   %sil                    ; n % d
movq    -72(%rbp), %r8          
shrw    $8, %ax                 ; move remainder to lower 8 bits
movb    %al, (%r8)              ; store the remainder
