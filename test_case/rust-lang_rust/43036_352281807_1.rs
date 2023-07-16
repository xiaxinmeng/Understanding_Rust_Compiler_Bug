c
struct Outer1 {
    struct Wrapper value1;
    struct Wrapper value2; 
};

struct Wrapper {
    uint8_t value[12];
};

struct Outer2 {
    uint8_t value1[12];
    uint8_t value2[12];
};
