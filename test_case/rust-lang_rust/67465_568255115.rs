rust
static PROMOTED: bool = unsafe { BoolTransmute { val: 3 }.bl };
const RAW_TRAIT_OBJ_CONTENT_INVALID: *const dyn Trait = &PROMOTED as *const _;
