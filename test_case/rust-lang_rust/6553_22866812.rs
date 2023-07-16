 rust
>>> L = []
>>> L.append(L)
>>> L == L
True
>>> x = [L, 1]
>>> y = [L, 2]
>>> x == y
False
