rust
option_a.and_not(option_b) = option_a.map(Ok).xor(option_b.map(Err)).and(option_a)
