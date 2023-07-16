	/*
	 * Set up the thread stack.
	 *
	 * Create a red zone below the main stack.  All other stacks
	 * are constrained to a maximum size by the parameters
	 * passed to mmap(), but this stack is only limited by
	 * resource limits, so this stack needs an explicitly mapped
	 * red zone to protect the thread stack that is just beyond.
	 */
	if (mmap(_usrstack - _thr_stack_initial -
	    _thr_guard_default, _thr_guard_default, 0, MAP_ANON,
	    -1, 0) == MAP_FAILED)
		PANIC("Cannot allocate red zone for initial thread");

	/*
	 * Mark the stack as an application supplied stack so that it
	 * isn't deallocated.
	 *
	 * XXX - I'm not sure it would hurt anything to deallocate
	 *       the main thread stack because deallocation doesn't
	 *       actually free() it; it just puts it in the free
	 *       stack queue for later reuse.
	 */
	thread->attr.stackaddr_attr = _usrstack - _thr_stack_initial;
	thread->attr.stacksize_attr = _thr_stack_initial;
	thread->attr.guardsize_attr = _thr_guard_default;
	thread->attr.flags |= THR_STACK_USER;

