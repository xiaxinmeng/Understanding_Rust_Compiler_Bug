 Rust
    /// An inverse of `min_capacity`.                                                                                                                                                        
    #[inline]
    fn usable_capacity(&self, cap: usize) -> usize {
        // As the number of entries approaches usable capacity,                                                                                                                                             
        // min_capacity(size) must be smaller than the internal capacity,                                                                                                                                   
        // so that the map is not resized:                                                                                                                                                                  
        // `min_capacity(usable_capacity(x)) <= x`.                                                                                                                                                         
        // The left-hand side can only be smaller due to flooring by integer                                                                                                                                
        // division.                                                                                                                                                                                        
        //                                                                                                                                                                                                  
        // This doesn't have to be checked for overflow since allocation size                                                                                                                               
        // in bytes will overflow earlier than multiplication by 10.                                                                                                                                        
        (cap * 10 + 10 - 1) / 11
    }
