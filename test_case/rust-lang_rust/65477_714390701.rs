rust
// Presumably we want to lint about overlap between the third and first
// branches, but not the second. This would require keeping track of which
// branches made a pattern redundant.
match x {
    (0 ..= 125, true, true) => {}
    (0 ..= 125, true, false) => {}
    (125 ..= 255, true, true) => {}
}
// If we also want a similar lint for this one, then this requires computing
// pattern intersections, which is something that the algorithm does not know
// how to do at all.
match x {
    (0 ..= 125, true, true) => {}
    (0 ..= 125, false, false) => {}
    (125 ..= 255, true, _) => {}
}
