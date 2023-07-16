rust
struct Map<T: 'static> {
    slice: &'static [T]
}

static FOO: Map<i8> = Map {
    slice: &[-1, -1, -1, -1 /* repeated N times */]
};
