
a) If x < y, totalOrder(x, y) is true.
b) If x > y, totalOrder(x, y) is false.
c) If x = y:
    1) totalOrder(−0, +0) is true.
    2) totalOrder(+0, −0) is false.
    3) If x and y represent the same floating-point datum:
        i) If x and y have negative sign, totalOrder(x, y) is true if and only if the exponent of x ≥ the exponent of y
        ii) otherwise totalOrder(x, y) is true if and only if the exponent of x ≤ the exponent of y.
d) If x and y are unordered numerically because x or y is NaN:
    1) totalOrder(−NaN, y) is true where −NaN represents a NaN with negative sign bit and y is a floating-point number.
    2) totalOrder(x, +NaN) is true where +NaN represents a NaN with positive sign bit and x is a floating-point number.
    3) If x and y are both NaNs, then totalOrder reflects a total ordering based on:
        i) negative sign orders below positive sign
        ii) signaling orders below quiet for +NaN, reverse for −NaN
        iii) lesser payload, when regarded as an integer, orders below greater payload for +NaN, reverse for −NaN.

Neither signaling NaNs nor quiet NaNs signal an exception. For canonical x and y, totalOrder(x, y) and totalOrder( y, x) are both true if x and y are bitwise identical.

NOTE — totalOrder does not impose a total ordering on all encodings in a format. In particular, it does not distinguish among different encodings of the same floating-point representation, as when one or both encodings are non-canonical.
