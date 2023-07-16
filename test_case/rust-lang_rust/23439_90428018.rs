 rust
use std::collections::BTreeMap;

pub trait Versioned {
    type Version: Ord;

    fn version(&self) -> Self::Version;
}

#[allow(dead_code)]
pub struct Controller<T: Versioned + ?Sized> {
    // Referencing `T::Version` causes the compiler error:
    migrations: BTreeMap<T::Version, Box<T>>
}

impl<T: Versioned + ?Sized> Controller<T> {
    pub fn new() -> Controller<T> {
        Controller { migrations: BTreeMap::new() }
    }
}

pub trait ExtendedVersioned : Versioned {
    type Version = i64;
}

#[allow(dead_code)]
pub struct ExtendedController {
    migrator: Controller<ExtendedVersioned>
}

impl ExtendedController {
    pub fn new() -> ExtendedController {
        ExtendedController { migrator: Controller::new() }
    }
}
