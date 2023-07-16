rust
// Arc::drop
    fn drop(&mut self) {
        if self.inner().strong.fetch_sub(1, Release) != 1 {
            return;
        }

        atomic::fence(Acquire);

        unsafe {
            self.drop_slow();
        }
    }

// Arc::drop_slow
    unsafe fn drop_slow(&mut self) {
        ptr::drop_in_place(&mut self.ptr.as_mut().data);

        if self.inner().weak.fetch_sub(1, Release) == 1 {
            atomic::fence(Acquire);
            Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()))
        }
    }

// Weak::drop
    fn drop(&mut self) {
        // If we find out that we were the last weak pointer, then its time to
        // deallocate the data entirely. See the discussion in Arc::drop() about
        // the memory orderings
        //
        // It's not necessary to check for the locked state here, because the
        // weak count can only be locked if there was precisely one weak ref,
        // meaning that drop could only subsequently run ON that remaining weak
        // ref, which can only happen after the lock is released.
        let inner: &ArcInner = if let Some(inner) = self.inner() { inner } else { return };

        if inner.weak.fetch_sub(1, Release) == 1 {
            atomic::fence(Acquire);
            unsafe { Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref())) }
        }
    }
