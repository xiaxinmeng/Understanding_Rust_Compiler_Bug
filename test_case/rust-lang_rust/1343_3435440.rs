
.LBB1_5:                                                                                                                                                                      
        #APP                                                                                                                                                                  
        ; let x = f(); (../src/test/run-pass/return-nil.rs:5:12: 5:23)                                                                                                        
        #NO_APP                                                                                                                                                               
.Ltmp8:                                                                                                                                                                       
        movl    -8(%ebp), %ebx                                                                                                                                                
        calll   _ZN1f17_cf204e937fd488eeE@PLT                                                                                                                                 
.Ltmp9:                                                                                                                                                                       
        jmp     .LBB1_7                                                                                                                                                       
.LBB1_7:                                                                                                                                                                      
        jmp     .LBB1_9
