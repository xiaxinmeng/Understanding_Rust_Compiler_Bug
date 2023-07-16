 
       39:  #APP
         40:  lgr %r2, %r13
check:84      X~~~~~~~~~~~~ error: no match found
         41:  #NO_APP
check:84     ~~~~~~~~
         42:  lmg %r13, %r15, 264(%r15)
check:84     ~~~~~~~~~~~~~~~~~~~~~~~~~~
         43:  br %r14
check:84     ~~~~~~~~
         44: .Lfunc_end1:
check:84     ~~~~~~~~~~~~
         45:  .size reg_i32, .Lfunc_end1-reg_i32
check:84     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
