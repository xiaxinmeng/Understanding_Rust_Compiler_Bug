rust
-  #![cfg_attr(not(test), no_std)]
+ // specify no_std does not mean that the std crate cannot be linked.
+ // https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
+ #![no_std]
