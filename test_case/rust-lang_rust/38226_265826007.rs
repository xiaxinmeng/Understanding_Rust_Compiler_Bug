rust
//! The following changes make the issue go away:
//!
//! - Removing type parameters
//! - Removing the let (returning the trait object works!)
//! - Making submod public
//! - Moving SomeTrait to the top-level
//!

pub fn foo<T>() {
    let _: Box<SomeTrait> = Box::new(SomeTraitImpl);
}

mod submod {
    pub trait SomeTrait {
        fn bar(&self) {
            debug_assert!(true);
        }
    }

}

use self::submod::SomeTrait;

pub struct SomeTraitImpl;
impl SomeTrait for SomeTraitImpl {}
