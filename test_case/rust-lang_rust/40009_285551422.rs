
error[E0308]: mismatched types
    --> src/libcollections/string.rs:1980:9
     |
1980 |         s.into_string()
     |         ^^^^^^^^^^^^^^^ expected struct `string::String`, found struct `std::string::String`
     |
     = note: expected type `string::String`
                found type `std::string::String`

error[E0308]: mismatched types
    --> src/libcollections/vec.rs:1903:9
     |
1903 |         s.into_vec()
     |         ^^^^^^^^^^^^ expected struct `vec::Vec`, found struct `std::vec::Vec`
     |
     = note: expected type `vec::Vec<T>`
                found type `std::vec::Vec<T>`
     = help: here are some functions which might fulfill your needs:
             - .remove(...)
             - .swap_remove(...)
