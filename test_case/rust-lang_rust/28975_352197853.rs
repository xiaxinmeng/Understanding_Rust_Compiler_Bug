rust
    // Currently we try hard to ensure that the call to `.exec()` doesn't
    // actually allocate any memory. While many platforms try to ensure that
    // memory allocation works after a fork in a multithreaded process, it's
    // been observed to be buggy and somewhat unreliable, so we do our best to
    // just not do it at all!
    //
    // Along those lines, the `argv` and `envp` raw pointers here are exactly
    // what's gonna get passed to `execvp`. The `argv` array starts with the
    // `program` and ends with a NULL, and the `envp` pointer, if present, is
    // also null-terminated.
    //
    // Right now we don't support removing arguments, so there's no much fancy
    // support there, but we support adding and removing environment variables,
    // so a side table is used to track where in the `envp` array each key is
    // located. Whenever we add a key we update it in place if it's already
    // present, and whenever we remove a key we update the locations of all
    // other keys.
