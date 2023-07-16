python
def items(f,s):
    return [f(x) for x in s.split()]

print(items(int,input("> ")))
