
.LBB1_5:                                                                                                                                                                      
        .loc    2 5 10                                                                                                                                                        
        #APP                                                                                                                                                                  
        ; let x = f(); (../src/test/run-pass/return-nil.rs:5:12: 5:23)                                                                                                        
        #NO_APP                                                                                                                                                               
        .loc    2 5 20                                                                                                                                                        
.Ltmp10:                                                                                                                                                                      
        movl    -8(%ebp), %ebx                                                                                                                                                
        calll   _ZN1f17_9f8ee824e170db46E@PLT                                                                                                                                 
        subl    $4, %esp                                                                                                                                                      
.Ltmp11:                                                                                                                                                                      
        jmp     .LBB1_7                                                                                                                                                       
.LBB1_7:                                                                                                                                                                      
        jmp     .LBB1_9
