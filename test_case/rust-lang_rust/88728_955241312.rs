rust
let nan0 = f32::NAN;
let nan1 = f32::from_bits(f32::NAN.to_bits() ^ 0x002a_aaaa);
let nan2 = f32::from_bits(f32::NAN.to_bits() ^ 0x0055_5555);
assert_eq!(nan0.next_down().to_bits(), nan0.to_bits());
assert_eq!(nan1.next_down().to_bits(), nan1.to_bits());
assert_eq!(nan2.next_down().to_bits(), nan2.to_bits()); // Fails.
