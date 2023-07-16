 rust
     fn fmt(&self, f: &mut Formatter) -> Result {
        // FIXME(#23542) Replace with type ascription.
        Pointer::fmt(&(*self as *const T), f)
    } 
