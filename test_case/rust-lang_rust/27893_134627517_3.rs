
    %discr: Option::Discr = foo.@discr ; match takes an lvalue
    switch %discr, None=>.none, Some=>.some
.some:
.some_body:
    ; foo_inner = foo.inner - not really a temporary, but good enough
    ; %f_arg = foo_inner
    (*%ret).as<Some>.0 = call f, foo.inner, onerr <snip> 
    *%ret = Some((*%ret).as<Some>.0) ; lowering ignores the self-assignment
    jmp exit
.none:
.none_body:
    *%ret = None
exit:
    ret

