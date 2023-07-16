
  2.55 │      → callq  rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step                                                                                                                              ▒
  2.35 │        movups 0x130(%rsp),%xmm0                                                                                                                                                                                                     ▒
[SNIP: instructions don't touch 0x20(%rsp)]
  3.22 │        movaps %xmm0,0x20(%rsp)   # ret value loaded from 0x20(%rsp)                                                                                                                                                                                                   ▒
 12.67 │        cmpb   $0x1,0x20(%rsp)    # ret value loaded _AGAIN_ from 0x20(%rsp)
