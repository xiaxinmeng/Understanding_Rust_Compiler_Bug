rust
        #![no_std]
        
        extern crate std;
        
        use ::std::prelude::v1::*;
        
        fn main ()
        {
            let _ = format_args!(""); // OK: available in the prelude.
            let _ = format!(""); // Error: available at `::std::*`
        }
        