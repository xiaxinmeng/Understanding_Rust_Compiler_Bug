 python
class Foo:
    def __init__(self, i):
        self.i = i

    def __eq__(self, other):
        return self.i == other.i
    def __hash__(self):
        return self.i
    def __repr__(self):
        return 'Foo(%d)' % self.i

d = { Foo(i): i for i in range(0, 10)}
print('** Before **')
print(d)
print('d[1]: %s' % d[Foo(1)])

for k in d:
    k.i = 9 - k.i

print('** After **')
print(d)
print('d[1]: %s' % d[Foo(1)])
