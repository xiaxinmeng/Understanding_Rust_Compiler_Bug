
error: lifetime parameter `'a` declared on fn `utils::transliterate` appears only in the return type, but here is required to be higher-ranked, which means that `'a` must appear in both argument and return types, #[forbid(hr_lifetime_in_assoc_type)] on by default
   --> src/comparison.rs:413:33
    |
413 |                                .filter_map(lowercase_if_alpha);
    |                                 ^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #33685 <https://github.com/rust-lang/rust/issues/33685>

error: lifetime parameter `'a` declared on fn `utils::transliterate` appears only in the return type, but here is required to be higher-ranked, which means that `'a` must appear in both argument and return types, #[forbid(hr_lifetime_in_assoc_type)] on by default
   --> src/comparison.rs:417:37
    |
417 |                                    .filter_map(lowercase_if_alpha);
    |                                     ^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #33685 <https://github.com/rust-lang/rust/issues/33685>

error: lifetime parameter `'a` declared on fn `utils::transliterate` appears only in the return type, but here is required to be higher-ranked, which means that `'a` must appear in both argument and return types, #[forbid(hr_lifetime_in_assoc_type)] on by default
   --> src/lib.rs:405:34
    |
405 |                                 .rev();
    |                                  ^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #33685 <https://github.com/rust-lang/rust/issues/33685>
