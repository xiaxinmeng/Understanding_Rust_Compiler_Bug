 rust
error[E0450]: cannot invoke tuple struct constructor with private fields
    --> ../src/libstd/collections/hash/map.rs:2081:9
     |
2081 |         DefaultHasher(SipHasher13::new_with_keys(self.k0, self.k1))
     |         ^^^^^^^^^^^^^ cannot construct with a private field
