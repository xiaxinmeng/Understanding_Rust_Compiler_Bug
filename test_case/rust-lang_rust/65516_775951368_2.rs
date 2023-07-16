diff
      pub fn new(…) -> Self {
          let pointee = …;
    -     let ptr = Box::into_raw(Box::new(pointee));
    +     let ptr = Arc::into_raw(Arc::new(pointee));
          impl Drop for WrapsAPtr { fn drop(&mut self) { unsafe {
               let Self(ptr) = *self;
    -          drop::<Box<Pointee>>(Box::from_raw(ptr as _));
    +          drop::<Arc<Pointee>>(Arc::from_raw(ptr as _));
          }}}
          WrapsAPtr(ptr)
      }
    