
start:
    ret = x
    tmp1 = y
    br *, a, b
a:
    goto exit
b:
    ret = tmp1
    goto exit
