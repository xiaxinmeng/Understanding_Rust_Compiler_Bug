Rust
// the first match arm should always be reachable, so I
// don't think this one is really needed
if false { goto before_guard_0; }
match x {
    unreachable:
        unreachable
    Some(w) =>
before_guard_0:
        if false { goto before_guard_1; }
        if guard() {
            ...
        } else {
            if true { goto next; } else { goto before_guard_1; }
        },
    x =>
before_guard_1:
        // yes, this fake edge can bind `y` when the current variant is
        // `None`, but this shouldn't break anything because the edge
        // is never actually taken.
        if false { goto before_guard_2; }
        ...
    Some(y) =>
before_guard_2:
        if false { goto before_guard_3; }
        if guard2(y) {
            ...
            break;
        } else {
            if true { goto next; } else { goto before_guard_3; }
        },
    z =>
before_guard_3:
        // just here for completeness - shouldn't have any effect
        if false { goto unreachable; }
        ...
}
