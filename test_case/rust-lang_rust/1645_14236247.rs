
struct {
    unsigned discriminant; // or whatever numeric type we use
    union {
        struct Variant1 { /* args of variant 1 */ };
        ...
        struct Variant1 { /* args of variant N */ };
    };
};
