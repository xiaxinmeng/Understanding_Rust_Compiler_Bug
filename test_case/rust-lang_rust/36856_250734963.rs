 asm
call g
xor al, -1
and al, 1
mov byte ptr [rbp - 10], al
call g
xor al, -1
mov cl, byte ptr [rbp - 10]
cmp cl, al
je correct
