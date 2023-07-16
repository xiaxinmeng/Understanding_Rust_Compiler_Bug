
Extended addition of `self + rhs + carry`. The booleans are interpreted as a single bit
integer of value 0 or 1. If unsigned overflow occurs, then the boolean in the tuple
returns 1. The output carry can be chained into the input carry of another carrying add,
which allows for arbitrarily large additions to be calculated.
