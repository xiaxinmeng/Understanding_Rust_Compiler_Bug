
    // We check that the local is live whenever it is used. Technically, violating this
    // restriction is only UB and not actually indicative of not well-formed MIR. This means
    // that an optimization which turns MIR that already has UB into MIR that fails this
    // check is not necessarily wrong. However, we have no such optimizations at the moment,
    // and so we include this check anyway to help us catch bugs. If you happen to write an
    // optimization that might cause this to incorrectly fire, feel free to remove this
    // check.
    