
start:
    tmp0 = x
    tmp1 = y
    br *, a, b
a:
    ret = tmp0
    goto exit
b:
    ret = tmp1
    goto exit
