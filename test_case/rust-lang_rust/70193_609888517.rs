
struct NeedsAlignment<A: maligned::Alignment> {
    ... some fields ...
    _aligned: [A; 0]
}
