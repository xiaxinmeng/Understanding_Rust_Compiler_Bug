
let iter = args.into_iter();
self.inner.reserve_args(iter.size_hint().0);
iter.for_each(|arg| {
    self.arg(arg.as_ref());
});
self
