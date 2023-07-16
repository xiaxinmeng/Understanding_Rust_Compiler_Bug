

So basically the initialization of `%rcx` is getting skipped by the incorrect landing pad, which in turn causes the crash.