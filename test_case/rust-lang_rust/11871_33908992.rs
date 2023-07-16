
function_foobar:
    load $(-1 * page_size)(%rsp), %rax
    load $(-2 * page_size)(%rsp), %rax
    ...
    load $(-stack_size)(%rsp), %rax
    < rest of function >
