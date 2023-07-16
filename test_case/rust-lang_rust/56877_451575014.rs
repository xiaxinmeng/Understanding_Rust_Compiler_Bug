C
struct Empty {};
struct Empty2 { struct Empty e; };
struct Empty3 { float z[0]; };
struct Empty4 { struct Empty3 e; };

union U1 {
    struct Empty s;
};

union U2 {
    struct Empty2 s;
};

union U3 {
    struct Empty3 s;
};

union U4 {
    struct Empty4 s;
};

// is an aggregate
struct Baz1 {
    float x;
    float y;
    union U1 u;
};

// is an aggregate
struct Baz2 {
    float x;
    float y;
    union U2 u;
};

// not an aggregate
struct Baz3 {
    float x;
    float y;
    union U3 u;
};

// not an aggregate
struct Baz4 {
    float x;
    float y;
    union U4 u;
};
