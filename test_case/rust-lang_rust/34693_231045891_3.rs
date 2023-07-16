
start:
    tmp0 = x
    ret = y
    br *, a, b
a:
    ret = tmp0
    goto exit
b:
    goto exit
