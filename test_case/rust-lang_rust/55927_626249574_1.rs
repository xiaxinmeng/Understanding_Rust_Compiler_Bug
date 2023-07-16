
warning: unnecessary braces around block return value
   --> src/compact_hash_map.rs:207:45
    |
207 |     static ref PRIME_SIEVE: primal::Sieve = { primal::Sieve::new(1_000_000) };
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these braces
    |
    = note: `#[warn(unused_braces)]` on by default
