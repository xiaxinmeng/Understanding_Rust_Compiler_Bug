rust
let expected_len_us = usize::try_from(expected_len)
    .expect("Only designed to run on 32-bit systems or higher");
