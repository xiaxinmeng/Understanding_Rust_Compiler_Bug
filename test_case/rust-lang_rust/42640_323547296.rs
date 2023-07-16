rust
#[allow(dead_code)]
impl PatKind {
    /// Returns true if the pattern is a reference pattern. A reference pattern
    /// is any pattern which can match a reference without coercion. Reference
    /// patterns include bindings, wildcards (_), consts of reference types, and
    /// patterns beginning with & or &mut. All other patterns are non-reference
    /// patterns.
    ///
    /// See https://github.com/rust-lang/rfcs/blob/master/text/2005-match-ergonomics.md#definitions
    /// for rationale.
    fn is_reference_pattern(&self) -> bool {
        // NB: intentionally don't use a catchall arm because it's good to be
        // forced to consider the below when adding/changing `PatKind`.
        //
        // FIXME: is the below correct? In particular, where do "consts of reference types"
        // end up?
        match *self {
            PatKind::Wild |
            PatKind::Binding(..) |
            PatKind::Ref(..) => true,
            PatKind::Struct(..) |
            PatKind::TupleStruct(..) |
            PatKind::Path(_) |
            PatKind::Tuple(..) |
            PatKind::Box(_) |
            PatKind::Lit(_) |
            PatKind::Range(..) |
            PatKind::Slice(..) => false,
        }
    }
}
