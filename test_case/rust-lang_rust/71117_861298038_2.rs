
define @caller() {
entry:
    %0 = alloca LargeStruct
    memset(%0, 0, 1024) ; _1.0 = [0;1024];

    ; call callee(copy _1) -> bb1;
    call @callee(%0)

    ; ...
}

define @callee(LargeStruct *%0) {
entry:
    %1 = gep %0, 0
    store %1, 42
    ret
}
