python
import sys
import random

def uniform(amplitude):
    return amplitude * (2 * random.random() - 1)

amp_1 = float(sys.argv[1])
amp_2 = float(sys.argv[2])

while True:
    a = uniform(amp_1)
    b = uniform(amp_2)
    if a != (a // b) * b + (a % b):
        print(a, b)
        sys.exit(1)
