
[ 1. Pre-match ]
       |
[ 2. Discriminant testing -- check discriminants ] <-+
       | (once a specific arm is chosen)             |
[ 3. Create "guard bindings" for arm ]               |
       |                                             |
[ 4. Execute guard code ] --(guard is false)---------+
       | (guard results in true)
[ 5. Create real bindings and execute arm ]
       |
[ Exit match ]
