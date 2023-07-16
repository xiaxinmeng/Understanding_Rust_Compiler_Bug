rust
let nan0 = f32::NAN.to_bits();
let nan1 = f32::NAN.to_bits() ^ 0x002a_aaaa;
let nan2 = f32::NAN.to_bits() ^ 0x0055_5555;
assert_eq!(f32::from_bits(nan0).next_down().to_bits(), nan0);
assert_eq!(f32::from_bits(nan1).next_down().to_bits(), nan1);
assert_eq!(f32::from_bits(nan2).next_down().to_bits(), nan2);
