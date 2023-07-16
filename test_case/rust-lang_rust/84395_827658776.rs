
; ModuleID = 'world.map.4'                                                                                                                                                                                          
source_filename = "566azmeytlkxgdp0"                                                                                                                                                                                
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"                                                                                                                        
target triple = "x86_64-unknown-linux-gnu"                                                                                                                                                                          
                                                                                                                                                                                                                    
; Function Attrs: uwtable                                                                                                                                                                                           
define dso_local i8* @__rust_alloc(i64 %0, i64 %1) local_unnamed_addr #0 {                                                                                                                                          
  %3 = tail call i8* @__rdl_alloc(i64 %0, i64 %1)                                                                                                                                                                   
  ret i8* %3                                                                                                                                                                                                        
}                                                                                                                                                                                                                   
                                                                                                                                                                                                                    
declare hidden i8* @__rdl_alloc(i64, i64) local_unnamed_addr                                                                                                                                                        
                                                                                                                                                                                                                    
; Function Attrs: uwtable                                                                                                                                                                                           
define dso_local void @__rust_dealloc(i8* %0, i64 %1, i64 %2) local_unnamed_addr #0 {                                                                                                                               
  tail call void @__rdl_dealloc(i8* %0, i64 %1, i64 %2)                                                                                                                                                             
  ret void                                                                                                                                                                                                          
}                                                                                                                                                                                                                   
                                                                                                                                                                                                                    
declare hidden void @__rdl_dealloc(i8*, i64, i64) local_unnamed_addr                                                                                                                                                
                                                                                                                                                                                                                    
; Function Attrs: uwtable                                                                                                                                                                                           
define dso_local i8* @__rust_realloc(i8* %0, i64 %1, i64 %2, i64 %3) local_unnamed_addr #0 {                                                                                                                        
  %5 = tail call i8* @__rdl_realloc(i8* %0, i64 %1, i64 %2, i64 %3)                                                                                                                                                 
  ret i8* %5                                                                                                                                                                                                        
}   
...                                                                        
