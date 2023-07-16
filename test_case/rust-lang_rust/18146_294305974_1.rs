
$ objdump -d ./libt.rlib
In archive ./libt.rlib:

t.0.o:     file format elf32-i386


Disassembly of section .text._ZN1t5other17h5beca376b5805208E:

00000000 <_ZN1t5other17h5beca376b5805208E>:
   0:	b8 ff ff 00 00       	mov    $0xffff,%eax
   5:	c3                   	ret

Disassembly of section .text._ZN1t9ext_other17he734743ecfa92ce9E:

00000000 <_ZN1t9ext_other17he734743ecfa92ce9E>:
   0:	b8 aa aa 00 00       	mov    $0xaaaa,%eax
   5:	c3                   	ret
objdump: rust.metadata.bin: File format not recognized
objdump: t.0.bytecode.deflate: File format not recognized
