rust
trait Fuse {
    type Fused;
    fn fuse(self) -> Self::Fused;
}

impl<T: Iterator> Fuse for T { ... }

impl<T: FusedIterator> Fuse for T { ... }
