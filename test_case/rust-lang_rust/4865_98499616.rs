 rust
pub mod a {
    use b::fn_b;
    use c::*;

    pub fn fn_a(){
    }
}

pub mod b {
    use a::fn_a;
    use c::*;

    pub fn fn_b(){
    }
}

pub mod c{
    pub fn fn_c(){
    }
}

use a::fn_a;
use b::fn_b;

fn main() {
}
