rust
    /// - To permit assignment into a local variable or other place
    ///   (including the "return slot") of type `!`.  This is allowed
    ///   if **either** the type of value being assigned is `!`, which
    ///   means the current code is dead, **or** the expression's
    ///   diverging flag is true, which means that a diverging value was
    ///   wrapped (e.g., `let x: ! = foo(return)`).
