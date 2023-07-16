
calle:                                  ! @calle
! %bb.0:
        save %sp, -208, %sp
        st %f0, [%fp+2031]
        add %fp, 2031, %i0
        or %i0, 4, %i0
        st %f1, [%i0]
        st %f2, [%fp+2039]
        st %f3, [%fp+2043]
        ld [%fp+2031], %f1
        call tst_use
        stx %i0, [%fp+2023]
        ldx [%fp+2023], %i0                     ! 8-byte Folded Reload
        call tst_use
        ld [%i0], %f1
        call tst_use
        ld [%fp+2039], %f1
        call tst_use
        ld [%fp+2043], %f1
        ret
        restore
