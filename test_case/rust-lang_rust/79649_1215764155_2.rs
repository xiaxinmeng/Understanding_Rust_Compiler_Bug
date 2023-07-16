
wink@3900x 22-08-15T19:53:58.046Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ diff *.s
1c1
< exper_code_coverage::if_short_circuit_and::if_short_circuit_and (src/if_short_circuit_and.rs:1):
---
> exper_code_coverage::short_circuit_and::short_circuit_and (src/short_circuit_and.rs:1):
10c10
<  jl      .LBB0_2
---
>  jl      .LBB1_2
12,13c12,13
<  jmp     .LBB0_3
< .LBB0_2:
---
>  jmp     .LBB1_3
> .LBB1_2:
20c20
< .LBB0_3:
---
> .LBB1_3:
