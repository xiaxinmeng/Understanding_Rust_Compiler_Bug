rust
#![feature(const_type_name)]

use std::future::Future;

const fn do_something_depends_on_name<F: Future>() -> &'static str {
    std::any::type_name::<F>()
}

// error[E0401]: can't use generic parameters from outer function
const fn do_something_depends_on_name_1<F: Future>() -> &'static str {
    let type_name: &str = std::any::type_name::<F>();   
    type_name
}

fn main() {
    
}
