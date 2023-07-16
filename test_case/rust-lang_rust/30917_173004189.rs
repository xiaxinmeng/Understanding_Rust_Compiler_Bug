 llvm
; version using get_unchecked()
while_body:                                                                                             
  %base.05 = phi i64 [ 0, %entry-block ], [ %base.1, %join ]                                            
  %lim.04 = phi i64 [ 3, %entry-block ], [ %11, %join ]                                                 
  %3 = lshr i64 %lim.04, 1                                                                              
  %4 = add i64 %base.05, %3                                                                             
  %5 = icmp eq i64 %4, 2                                                                                
  %6 = icmp ult i64 %4, 2                                                                               
  %..i.i = select i1 %6, i8 -1, i8 1                                                                    
  %sret_slot.0.i.i = select i1 %5, i8 0, i8 %..i.i                                                      
  switch i8 %sret_slot.0.i.i, label %match_else [                                                       
    i8 0, label %match_case                                                                             
    i8 -1, label %match_case3                                                                           
    i8 1, label %join                                                                                   
  ]                                                                                                     

; version using safe indexing                                                                                                 
while_body:                                                                                             
  %base.07 = phi i64 [ 0, %entry-block ], [ %base.1, %join ]                                            
  %lim.06 = phi i64 [ 3, %entry-block ], [ %12, %join ]                                                 
  %3 = lshr i64 %lim.06, 1                                                                              
  %4 = add i64 %base.07, %3                                                                             
  %5 = icmp ugt i64 %4, 2                                                                               
  br i1 %5, label %cond.i, label %"_ZN3bar16_$LT$closure$GT$12closure.3826E.exit", !prof !0             

cond.i:                                                                                                 
  %.lcssa = phi i64 [ %4, %while_body ]                                                                 
  tail call void @_ZN9panicking18panic_bounds_check20hd44ea11c616af168XYLE(...)        
  unreachable                                                                                           

"_ZN3bar16_$LT$closure$GT$12closure.3826E.exit":                                                        
  %6 = icmp eq i64 %4, 2                                                                                
  %7 = icmp ult i64 %4, 2                                                                               
  %..i.i = select i1 %7, i8 -1, i8 1                                                                    
  %sret_slot.0.i.i = select i1 %6, i8 0, i8 %..i.i                                                      
  switch i8 %sret_slot.0.i.i, label %match_else [                                                       
    i8 0, label %match_case                                                                             
    i8 -1, label %match_case3                                                                           
    i8 1, label %join                                                                                   
  ]                                                                                                     
