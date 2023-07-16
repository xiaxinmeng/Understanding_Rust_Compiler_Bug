rust
            #![cfg_attr(all(), feature(no_core), no_core)]
            
            extern crate core;
            
            use ::core::prelude::v1::*;
            
            fn _check()
            {
                let _ = concat!(""); // OK
                let _ = panic!(""); // Error
            }
            