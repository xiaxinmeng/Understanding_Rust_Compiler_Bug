
    ; Create a terminator entry for the `.eh_frame` section of rust binaries.
    ;
    ; See https://github.com/rust-lang/rust/issues/47551 for details
    ;
    ; You can build this with:
    ;
    ;    nasm -f elf64 eh_frame_terminator.asm
    ;
    section .eh_frame
      ; The terminator is a 0-length CIE, the first field of which is the length
      ; as a 32-bit number.
      dd 0x00000000
    