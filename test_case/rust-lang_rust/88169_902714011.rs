
loop1:
.Lloop:
    jmp .Lloop

loop2:
.Lloop:
    jmp .Lloop

main:
    jmp loop1
    jmp loop2
