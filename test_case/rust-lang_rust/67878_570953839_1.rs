
  7.37 │      → callq  rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step                                                                                                                              ▒
 11.35 │        movzbl 0x20(%rsp),%edx  # ret value loaded from 0x20(%rsp)                                                                                                                                                                                             ▒
[SNIP: instructions don't touch %edx]
  5.15 │        cmp    $0x1,%dl         # ret value used for comparison                                                                                                                                                                                                           ▒
