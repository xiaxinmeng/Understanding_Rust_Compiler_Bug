rust
> impl<'a, L: Latch> Latch for TickleLatch<'a, L> {
>     #[inline]
>     fn set(&self) {
>         self.inner.set();
>         self.sleep.tickle(usize::MAX);
>     }
> }
> 