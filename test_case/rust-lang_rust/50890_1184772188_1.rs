bash
 $ rustc +nightly --print target-features -Z unstable-options --target mips64-unknown-linux-gnuabi64
Features supported by rustc for this target:
    fp64                     - Support 64-bit FP registers.
    msa                      - Mips MSA ASE.
    crt-static               - Enables C Run-time Libraries to be statically linked.

Code-generation features supported by LLVM for this target:
...
    mt                       - Mips MT ASE.
    nan2008                  - IEEE 754-2008 NaN encoding.
    noabicalls               - Disable SVR4-style position-independent code.
    nomadd4                  - Disable 4-operand madd.fmt and related instructions.
    nooddspreg               - Disable odd numbered single-precision registers.
    p5600                    - The P5600 Processor.
    ptr64                    - Pointers are 64-bit wide.
    single-float             - Only supports single precision float.
    soft-float               - Does not support floating point instructions.
...
