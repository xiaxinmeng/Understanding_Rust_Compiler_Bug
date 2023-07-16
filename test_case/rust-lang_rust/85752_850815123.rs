
[ a , b , c ]
        ^--- 2..2
        ^^^^^--- 2..3 same as 2..2 but moves the end index to the right
            ^--- 3..3 same as 2..3 but moves the start index to the right. 
                      This can't possibly be out of bounds if both 2..2 and 2..3 are considered in bounds.
