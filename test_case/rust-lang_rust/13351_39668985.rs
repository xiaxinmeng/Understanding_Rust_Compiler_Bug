
    // remove lock_cnt from the struct

    #[inline]
    pub fn doit(&self, f: ||) {
        // Implementation-wise, this would seem like a fairly trivial primitive.
        // The stickler part is where our mutexes currently require an
        // allocation, and usage of a `Once` shouldn't leak this allocation.
        //
        // This means that there must be a deterministic destroyer of the mutex
        // contained within (because it's not needed after the initialization
        // has run).
        //
        // The general algorithm here is we keep a state value that indicates whether
        // we've finished the work. This lets us avoid the expensive atomic
        // read-modify-writes and mutex if there's no need. If we haven't finished
        // the work, then we use a second value to keep track of how many outstanding
        // threads are trying to use the mutex. When the last thread releases the
        // mutex it drops the lock count to a "very negative" value to indicate to
        // other threads that the mutex is gone.

        let state = self.state.load(atomics::Acquire);
        if state >= 0 {
            self.doit_slow(f)
        }
    }

    #[inline(never)]
    fn doit_slow(&self, f: ||) {
        // If the count is negative, then someone else finished the job,
        // otherwise we run the job and record how many people will try to grab
        // this lock
        let HIGH_BIT = (uint::MAX >> 1) + 1;
        let DONE_BIT = HIGH_BIT as int;
        let STARTED_BIT = (HIGH_BIT >> 1) as int;
        let LOCK_DONE_BIT = (HIGH_BIT >> 2) as int;
        let LOCK_MASK = (LOCK_DONE_BIT - 1) as int;

        if (self.state.fetch_add(1, atomics::Acquire) & LOCK_DONE_BIT) != 0 {
            // Make sure we never overflow.
            self.state.store(DONE_BIT | STARTED_BIT | LOCK_DONE_BIT, atomics::Relaxed);
            return
        }

        let guard = self.mutex.lock();
        if (self.state.fetch_or(STARTED_BIT, atomics::Relaxed) & STARTED_BIT) == 0 {
            // we're the first one here
            f();
            self.state.fetch_or(DONE_BIT, atomics::Release);
        }
        drop(guard);

        // Last one out cleans up after everyone else, no leaks!
        // here STARTED_BIT and DONE_BIT are always set
        if (self.state.fetch_add(-1, atomics::Release) == (DONE_BIT | STARTED_BIT | 1) {
            // we just decremented it to 0, now make sure we can drop it to int::MIN.
            // If this fails, someone else just waltzed in and took the mutex
            if self.state.compare_and_swap(DONE_BIT | STARTED_BIT, DONE_BIT | STARTED_BIT | LOCK_DONE_BIT, atomics::AcqRel) == (DONE_BIT | STARTED_BIT) {
                // good, we really were the last ones out
                unsafe { self.mutex.destroy() }
            }
        }
    }
