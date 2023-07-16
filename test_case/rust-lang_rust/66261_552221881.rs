rust
unsafe trait Trait {
    type Assoc;
}

use std::{
    mem::{self, ManuallyDrop},
};

#[allow(unused)]
fn func<T: Trait>(slice: Vec<T::Assoc>) {
    unsafe {
        let transmute = mem::transmute;
        let _: Vec<ManuallyDrop<T::Assoc>> = transmute(slice);
    }
}
