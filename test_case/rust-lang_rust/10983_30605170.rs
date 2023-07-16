 c
struct List {
    u8 discriminant;
    union {
        struct { T elem; List* next; } cons;
        zero_sized_type nil;
    } variants;
}
