
const C: u8 = 10;
let C @ SUBPATTERN = 11; // OK, no error, C is interpreted as a fresh binding, because it cannot be anything else syntactically
