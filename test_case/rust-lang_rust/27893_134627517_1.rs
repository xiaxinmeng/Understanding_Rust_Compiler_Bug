
    %discr: Option::Discr = foo.@discr ; match takes an lvalue
    switch %discr, None=>.none, Some=>.some
.some:
    foo_inner = foo.inner ; this isn't even a temporary - borrowck ensures no trouble
.some_body:
    %f_arg : T = foo_inner
    %f_ret : U = call f, %f_arg, onerr <snip> 
    %ret_val_2 : Option<U> = Some(%f_ret)
    *%ret = %ret_val_2
    jmp exit
.none:
.none_body:
    %ret_val : Option<U> = None
    *%ret = %ret_val
exit:
    ret
