
$cat testabi.rs 
#![crate_type="lib"]

pub fn add1(x: f64) -> f64 {
    x + 1.0
}
$./rust/i686-unknown-linux-gnu/stage2/bin/rustc -O --target=i686-unknown-linux-gnu -C target-cpu=pentium4 testabi.rs 
$objdump -S libtestabi.rlib 
In archive libtestabi.rlib:

testabi.0.o:     file format elf32-i386


Disassembly of section .text._ZN4add120hfa2a4733581ccd0deaaE:

00000000 <_ZN4add120hfa2a4733581ccd0deaaE>:
   0:   83 ec 0c                sub    $0xc,%esp
  13:   e8 00                   and    $0x10,%al
  15:   00 00 00 58 81 c0 03    addsd  0x0(%eax),%xmm0
  1c:   00 
  1d:   00 00 f2 0f 10          movsd  %xmm0,(%esp)
  22:   44 24 10                fldl   (%esp)
  25:   f2 0f 58                add    $0xc,%esp
  28:   80                      ret    
  29:   Address 0x0000000000000029 is out of bounds.

objdump: rust.metadata.bin: File format not recognized
objdump: testabi.0.bytecode.deflate: File format not recognized
$./rust/i686-unknown-linux-gnu/stage2/bin/rustc -O --target=i686-unknown-linux-gnu -C target-cpu=i686 testabi.rs 
$objdump -S libtestabi.rlib 
In archive libtestabi.rlib:

testabi.0.o:     file format elf32-i386


Disassembly of section .text._ZN4add120hfa2a4733581ccd0deaaE:

00000000 <_ZN4add120hfa2a4733581ccd0deaaE>:
   0:   d9 e8                   fld1   
   2:   dc 44 24 04             faddl  0x4(%esp)
   6:   c3                      ret    
objdump: rust.metadata.bin: File format not recognized
objdump: testabi.0.bytecode.deflate: File format not recognized
$
