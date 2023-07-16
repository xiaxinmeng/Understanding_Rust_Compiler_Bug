
>>> range(10)[::0]
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: slice step cannot be zero
>>> range(10)[::1]
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
>>> range(10)[::2]
[0, 2, 4, 6, 8]
