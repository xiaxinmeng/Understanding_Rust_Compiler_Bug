asm
_ZN4uirc7command10IrcCommand3new17hca225b68bb9fc6afE:
        .cfi_startproc
        cmpq    $3, %rdx                /* is slice size = 3 */
        jne     .LBB14_1                /* if not, branch to << not a numeric >> */
        movb    2(%rsi), %r8b           /* put slice[2] in r8b */
        addb    $-48, %r8b              /* it's a value in b'0' to b'9', normalize to 0u8 to 9u8, smaller values overflow into very large values but we can use this property next */
        cmpb    $9, %r8b                /* since our b'0' to b'9' has been turned into 0u8 to 9u8, we can just compare for values between 0 and 9 */
        ja      .LBB14_1                /* and branch if it's not within range */
        movb    (%rsi), %cl             /* repeat the same for slice[0] */
        addb    $-48, %cl
        cmpb    $9, %cl
        ja      .LBB14_1
        movb    1(%rsi), %al            /* repeat it for slice[1] */
        addb    $-48, %al
        cmpb    $9, %al
        ja      .LBB14_1
        movzbl  %cl, %ecx               /* moving things around, zero-extending */
        movzbl  %al, %eax
        leal    (%rcx,%rcx,4), %ecx     /* this is actually a multiply by 5 */
        leal    (%rax,%rcx,2), %eax     /* and this is a multiply by 2 with an add */
        movzbl  %r8b, %ecx
        leal    (%rax,%rax,4), %eax     /* multiply by 5 */
        leal    (%rcx,%rax,2), %eax     /* multiply by 2 with an add */
        movq    $1, (%rdi)              /* set the variant (IrcCommand::Numeric) */
        movw    %ax, 8(%rdi)            /* Numeric the struct is Numeric<'a>(u16, &'a [u8; 3]). load up the u16 part of it */
        movq    %rsi, %rdx              /* shuffle things around so we can reuse the code below */
        jmp     .LBB14_6                /* go to << common path >> */
.LBB14_1:                               /* << not a numeric >> */
        movq    $0, (%rdi)              /* set the variant (IrcCommand::Stringy) */
        movq    %rsi, 8(%rdi)           /* &'a [u8] is a fat pointer, load it's contents. */
.LBB14_6:                               /* << common path >> */
        movq    %rdx, 16(%rdi)          /* load the contents (for Numeric) or length (for Stringy) */
        movq    %rdi, %rax              /* put result in "return slot" */
        retq                            /* return */
.Lfunc_end14:
        .size   _ZN4uirc7command10IrcCommand3new17hca225b68bb9fc6afE, .Lfunc_end14-_ZN4uirc7command10IrcCommand3new17hca225b68bb9fc6afE
        .cfi_endproc
