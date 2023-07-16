
from itertools import islice

def rnd(ia = 3877, ic = 29573, im = 139968):
    seed = 42
    imf = float(im)
    while 1:
        seed = (seed * ia + ic) % im
        r = seed / imf
        yield "ACGT"[int(r * 4)]

data = "".join(islice(rnd(), 0, 120000))
file("data.txt", "w").write(data)
