rust
#![feature(let_chains)]

use syn::{Attribute, Meta};

fn f(attr: &Attribute) -> bool {
    if let Meta::NameValue(name_value) = attr.parse_meta().ok().unwrap()
        && let path_idents = name_value.path.segments.iter()
        /* other stuff */
    {
        
    }
    
    true
}
