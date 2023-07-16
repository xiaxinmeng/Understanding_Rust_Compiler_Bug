 asm
====
1:30c # 1GB - negative offset
    movq    %rsi, -1073741784(%rbp)
2:31c # 2GB - negative offset
    movq    %rsi, -2147483608(%rbp)
3:31c # 3GB - positive offset (wrong side of stack beginning)
    movq    %rsi, 1073741864(%rbp)
====
