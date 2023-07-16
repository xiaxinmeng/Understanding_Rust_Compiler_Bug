rust
#![feature(coerce_unsized)]
use core::ops::CoerceUnsized;

trait Trait {}

struct B<T>(Box<T>);

impl<T> CoerceUnsized<B<dyn Trait>> for B<T>
where T: Trait {}
