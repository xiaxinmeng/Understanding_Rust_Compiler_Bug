
** Before **
{Foo(0): 0, Foo(1): 1, Foo(2): 2, Foo(3): 3, Foo(4): 4, Foo(5): 5, Foo(6): 6, Foo(7): 7, Foo(8): 8, Foo(9): 9}
d[1]: 1
** After **
{Foo(9): 0, Foo(8): 1, Foo(7): 2, Foo(6): 3, Foo(5): 4, Foo(4): 5, Foo(3): 6, Foo(2): 7, Foo(1): 8, Foo(0): 9}
Traceback (most recent call last):
  File "modify_key.py", line 22, in <module>
    print('d[1]: %s' % d[Foo(1)])
KeyError: Foo(1)
