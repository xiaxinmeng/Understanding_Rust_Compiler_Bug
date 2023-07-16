
f32::from_bits(bits.try_into().unwrap_or_else(|_| unreachable!()))
