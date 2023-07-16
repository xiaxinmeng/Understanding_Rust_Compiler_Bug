C
// still an homogeneous aggregate, despite having a non-0-sized struct and float.
struct Foo2 { float t; };
struct Baz2 {
    struct Foo2 x;
    float y[1];
};

float sizeof_baz2(struct Baz2 b) {
    return b.y[0];
}
