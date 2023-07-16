rust
            #![cfg_attr(all(), feature(alloc_prelude), no_std)]
            
            extern crate alloc;
            
            use ::alloc::prelude::v1::vec; // Error, it's at `::alloc::vec`.
            
            fn main ()
            {
                let _ = vec![]; // Error
            }
            