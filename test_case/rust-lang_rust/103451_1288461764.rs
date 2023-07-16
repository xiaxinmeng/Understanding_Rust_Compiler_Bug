rust
struct R { }

struct S {
    x: [u8; R //~ ERROR this file contains an unclosed delimiter
    //~| ERROR expected value
