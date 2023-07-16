
    /// Invoked when a strand represents an **answer**. This means
    /// that the strand has no subgoals left. There are two possibilities:
    ///
    /// - the strand may represent an answer we have already found; in
    ///   that case, we can return `StrandFail::NoSolution`, as this
    ///   strand led nowhere of interest.
    /// - the strand may represent a new answer, in which case it is
    ///   added to the table and `Ok` is returned.
