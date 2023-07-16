
    /// Creates an empty vector with capacity for `cap` values.                 
    pub fn with_capacity(cap: usize) -> Self {                                  
        let raw = RawVec::with_capacity(cap);                                   
        for i in 0..raw.capacity() {                                            
            unsafe {                                                            
                ptr::write(raw.ptr().offset(i as isize), T::default());         
            }                                                                   
        }                                                                       
        DefaultVec { raw }                                                      
    }
