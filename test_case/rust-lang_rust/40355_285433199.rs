rust
struct Map {
    slice: &'static [i8]
}

static FOO: Map = Map {
    slice: &[-1, -1, -1, -1 /* repeated N times */]
};
