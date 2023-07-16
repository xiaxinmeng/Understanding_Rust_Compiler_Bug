
    %discr: Option::Discr = foo.@discr ; match takes an lvalue
    switch %discr, None=>.none, Some=>.some
.some:
    foo_inner = foo.inner
.some_body:
    %f_arg : Option<T> = foo_inner
    ; %ret_val_2.as<Some>.0 = %f_ret
    ; (*%ret) = %ret_val_2
    (*%ret).as<Some>.0 = call f, %f_arg, onerr <snip> 
    *%ret = Some((*%ret).as<Some>.0) ; lowering ignores the self-assignment
    jmp exit
.none:
.none_body:
    *%ret = None
exit:
    ret
