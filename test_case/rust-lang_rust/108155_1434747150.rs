rs
#![allow(deref_into_dyn_supertrait)]
trait Trait {}
impl std::ops::Deref for dyn Trait + Send + Sync {
    type Target = dyn Trait;
    fn deref(&self) -> &Self::Target {
        self
    }
}
