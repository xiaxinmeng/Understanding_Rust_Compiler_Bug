
callee:
        .cfi_startproc
        save %sp, -192, %sp
        .cfi_def_cfa_register %fp
        .cfi_window_save
        .cfi_register %o7, %i7
        st %f3, [%fp+2043]
        st %f2, [%fp+2039]
        st %f1, [%fp+2035]
        fmovs %f0, %f1
        call tst_use
        nop
        call tst_use
        ld [%fp+2035], %f1
        call tst_use
        ld [%fp+2039], %f1
        call tst_use
        ld [%fp+2043], %f1
        ret
        restore
