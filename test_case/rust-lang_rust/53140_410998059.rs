rust
mod mod1 {
    pub struct Item(usize);
    
    use Item as Something; // ERROR
}

use mod1::*;

mod mod2 {
    use Item; // OK
}
