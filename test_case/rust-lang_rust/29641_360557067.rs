rust
match act {
    Subcondition(box Always(act)) => simplify(act),
    Subcondition(box cond) => Subcondition(Box::new(simplify(cond))),
    otherwise => otherwise
}
