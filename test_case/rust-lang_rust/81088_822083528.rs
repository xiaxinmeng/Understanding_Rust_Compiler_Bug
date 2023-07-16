
.macro some_macro
        jmp .Lmy_label_\@
.Lmy_label_\@:
        nop
.endm
