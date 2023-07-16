 llvm
%X = type { i8, i8 }                                  

declare void @foo()                                   

define void @f(%X* noalias) {                         
  %x = alloca %X*                                     
  store %X* %0, %X** %x                               
  %2 = load %X*, %X** %x                              
  %3 = getelementptr inbounds %X, %X* %2, i32 0, i32 0
  store i8 1, i8* %3                                  
  call void @foo()                                    
  %4 = load %X*, %X** %x                              
  %5 = getelementptr inbounds %X, %X* %4, i32 0, i32 0
  store i8 0, i8* %5                                  
  ret void                                            
}                                                     
