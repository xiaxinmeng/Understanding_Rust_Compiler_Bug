 
    union LiteralKindData {
        struct Int { Base base; bool empty_int; } Int;
        struct Float { Base base; bool empty_exponent; } Float;
    } data;
