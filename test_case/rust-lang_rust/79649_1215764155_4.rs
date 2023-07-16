
$ fd --glob '*.s' target
target/release/deps/exper_code_coverage-865a6210cdb565bc.s
target/release/deps/exper_code_coverage-c551574da956536f.s
wink@3900x 22-08-15T20:02:02.209Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cat -n target/release/deps/exper_code_coverage-865a6210cdb565bc.s
     1          .text
     2          .file   "exper_code_coverage.210b8923-cgu.0"
     3          .section        .text._ZN19exper_code_coverage17short_circuit_and17short_circuit_and17h626fd720e3a99931E,"ax",@progbits
     4          .globl  _ZN19exper_code_coverage17short_circuit_and17short_circuit_and17h626fd720e3a99931E
     5          .p2align        4, 0x90
     6          .type   _ZN19exper_code_coverage17short_circuit_and17short_circuit_and17h626fd720e3a99931E,@function
     7  _ZN19exper_code_coverage17short_circuit_and17short_circuit_and17h626fd720e3a99931E:
     8          .cfi_startproc
     9          cmpl    %esi, %edi
    10          setl    %sil
    11          cmpl    %ecx, %edx
    12          setl    %al
    13          andb    %sil, %al
    14          retq
    15  .Lfunc_end0:
    16          .size   _ZN19exper_code_coverage17short_circuit_and17short_circuit_and17h626fd720e3a99931E, .Lfunc_end0-_ZN19exper_code_coverage17short_circuit_and17short_circuit_and17h626fd720e3a99931E
    17          .cfi_endproc
    18
    19          .globl  _ZN19exper_code_coverage20if_short_circuit_and20if_short_circuit_and17h996a8c765e790e2dE
    20          .type   _ZN19exper_code_coverage20if_short_circuit_and20if_short_circuit_and17h996a8c765e790e2dE,@function
    21  .set _ZN19exper_code_coverage20if_short_circuit_and20if_short_circuit_and17h996a8c765e790e2dE, _ZN19exper_code_coverage17short_circuit_and17short_circuit_and17h626fd720e3a99931E
    22          .section        ".note.GNU-stack","",@progbits
