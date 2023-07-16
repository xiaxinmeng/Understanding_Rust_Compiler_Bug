rust
const OTHER: [bool; 1] = [true];

const MYCONST_HELPER: u8 = {
    OTHER[0] = false; //~ warning
    0
};

#[allow(const_item_mutation)]
const MYCONST: u8 = MYCONST_HELPER;
