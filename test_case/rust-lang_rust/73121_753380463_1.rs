rust
const NZ1: NonZeroU32 = NonZeroU32::new(1).unwrap();
const NZ1000: NonZeroU32 = NonZeroU32::new(1000).unwrap();

(NZ1 .. NZ1000)
.map(...)
.filter(pred)
...
