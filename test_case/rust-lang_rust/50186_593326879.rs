rust
if self.inner.counter().strong.update(|x| x - 1) == 0 {
    ...
}
