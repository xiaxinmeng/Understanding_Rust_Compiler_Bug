
span_lint_and_sugg(
    cx,
    USELESS_VEC,
    span,
    "useless use of `vec!`",
    "you can use a slice directly",
    "consider making the value a reference",
    snippet,
    applicability,
);
