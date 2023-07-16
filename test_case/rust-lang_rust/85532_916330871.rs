
Extended subtraction of `self - rhs - borrow`. The "borrowing" here refers to borrowing in the full subtractor sense.
The booleans are interpreted as a single bit integer of value 0 or 1. If unsigned overflow occurs, then the boolean
in the tuple returns 1. The output carry can be chained into the input carry of another borrowing subtract,
which allows for arbitrarily large subtraction to be calculated.
