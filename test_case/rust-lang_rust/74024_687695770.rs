
If the length is odd, say n * 2 + 1, then half == n.
The full slice has n elements on each size of mid:
|--n--|mid|--n--|
If cmp == Less, the new slice is:
      |mid|--n--|
If cmp == Greater, the new slice is:
|--n--|mid|

If the length is even, say n * 2, then half == n.
The full slice has n elements to the left of mid and n - 1 to the right:
|--n--|mid|--(n-1)--|
If cmp == Less, the new slice is:
      |mid|--(n-1)--|
If cmp == Greater, the new slice is:
|--n--|
