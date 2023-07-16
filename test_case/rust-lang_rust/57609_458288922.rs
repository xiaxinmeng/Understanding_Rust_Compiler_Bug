
[ 1. Pre-match ]
[ (create fake borrows) ]
       |
[ 2. Discriminant testing -- check discriminants ] <-+
       |                                             |
       | (once a specific arm is chosen)             |
       |                                             |
[ (read fake borrows) ]                              |
[ 3. Create "guard bindings" for arm ]               |
[ (recreate fake borrows) ]                          |
       |                                             |
[ 4. Execute guard code ] --(guard is false)---------+
       |
       | (guard results in true)
       |
[ (read fake borrows) ]
[ 5. Create real bindings and execute arm ]
       |
[ Exit match ]
