rust
    fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
        &mut arg[..] //[lexical]~^ ERROR: variable does not need to be mutable
                     //[nll]~^ ERROR: variable does not need to be mutable
    }
