text
[ 1. Pre-match ]
[ (create fake borrows) ] (*)
       |
[ 2. Discriminant testing -- check discriminants ] <-+
       |                                             |
       | (once a specific arm is chosen)             |
       |                                             |                            |
[ (read fake borrows) ]                              |
[ 3. Create "guard bindings" for arm (optional) ]    |
[ (recreate fake borrows ) ] (**)                    |
       |                                             |
[ 4. Execute guard code (opt) ] --(guard is false)---+
       |
       | (guard results in true)
       |
[ ( no fake read ) ] (+)
[ 5. Create real bindings and execute arm ]
       |
[ Exit match ]
