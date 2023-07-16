
struct BigEndianU32(...);
let array: Vec<BigEndianU32>;
let array: Vec<u32> = array.map_inplace(|x| x.to_u32());
