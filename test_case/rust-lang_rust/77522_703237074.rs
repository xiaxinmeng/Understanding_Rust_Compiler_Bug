rust
const OTHER: [bool; 1] = [true];

#[allow(const_item_mutation)]
const MYCONST: u8 = {
    OTHER[0] = false; // doesn't warn
    0
};
