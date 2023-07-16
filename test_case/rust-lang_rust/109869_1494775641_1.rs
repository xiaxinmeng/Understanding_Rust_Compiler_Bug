rust
#![feature(trivial_bounds)]
trait Empty {}

impl Default for dyn Empty
where
    Self: Sized,
{
    fn default() -> Self {
        ()
    }
}
