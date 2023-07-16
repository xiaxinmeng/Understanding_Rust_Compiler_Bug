 rust
#![feature(core_intrinsics)]
extern crate style;
use std::intrinsics::discriminant_value;

pub fn with_match(decl: &style::properties::PropertyDeclaration) -> usize {
    decl.discriminant_value()
}

pub fn with_intrinsic(decl: &style::properties::PropertyDeclaration) -> usize {
    unsafe { discriminant_value(decl) as usize }
}
