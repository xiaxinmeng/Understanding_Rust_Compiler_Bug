
[01:09:14] error[E0308]: mismatched types
[01:09:14]    --> tools/rls/src/build/cargo.rs:188:44
[01:09:14]     |
[01:09:14] 188 |     let spec = Packages::from_flags(false, &[], &packages)?;
[01:09:14]     |                                            ^^^
[01:09:14]     |                                            |
[01:09:14]     |                                            expected struct `std::vec::Vec`, found &[_; 0]
[01:09:14]     |                                            help: try using a conversion method: `&[].to_vec()`
[01:09:14]     |
[01:09:14]     = note: expected type `std::vec::Vec<std::string::String>`
[01:09:14]                found type `&[_; 0]`
[01:09:14]
[01:09:14] error[E0308]: mismatched types
[01:09:14]    --> tools/rls/src/build/cargo.rs:188:49
[01:09:14]     |
[01:09:14] 188 |     let spec = Packages::from_flags(false, &[], &packages)?;
[01:09:14]     |                                                 ^^^^^^^^^
[01:09:14]     |                                                 |
[01:09:14]     |                                                 expected struct `std::vec::Vec`, found reference
[01:09:14]     |                                                 help: consider removing the borrow: `packages`
[01:09:14]     |
[01:09:14]     = note: expected type `std::vec::Vec<std::string::String>`
[01:09:14]                found type `&std::vec::Vec<std::string::String>`
[01:09:14]
[01:09:14] error[E0308]: mismatched types
[01:09:14]    --> tools/rls/src/build/cargo.rs:191:17
[01:09:14]     |
[01:09:14] 191 |         target: opts.target.as_ref().map(|t| &t[..]),
[01:09:14]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::string::String`, found &str
[01:09:14]     |
[01:09:14]     = note: expected type `std::option::Option<std::string::String>`
[01:09:14]                found type `std::option::Option<&str>`
[01:09:14]
[01:09:14] error[E0308]: mismatched types
[01:09:14]    --> tools/rls/src/build/cargo.rs:195:13
[01:09:14]     |
[01:09:14] 195 |             &opts.bin,
[01:09:14]     |             ^^^^^^^^^
[01:09:14]     |             |
[01:09:14]     |             expected struct `std::vec::Vec`, found reference
[01:09:14]     |             help: consider removing the borrow: `opts.bin`
[01:09:14]     |
[01:09:14]     = note: expected type `std::vec::Vec<std::string::String>`
[01:09:14]                found type `&std::vec::Vec<std::string::String>`
[01:09:14]
[01:09:15] error[E0308]: mismatched types
[01:09:15]    --> tools/rls/src/build/cargo.rs:198:13
[01:09:15]     |
[01:09:15] 198 |             &[],
[01:09:15]     |             ^^^
[01:09:15]     |             |
[01:09:15]     |             expected struct `std::vec::Vec`, found &[_; 0]
[01:09:15]     |             help: try using a conversion method: `&[].to_vec()`
[01:09:15]     |
[01:09:15]     = note: expected type `std::vec::Vec<std::string::String>`
[01:09:15]                found type `&[_; 0]`
[01:09:15]
[01:09:15] error[E0308]: mismatched types
[01:09:15]    --> tools/rls/src/build/cargo.rs:200:13
[01:09:15]     |
[01:09:15] 200 |             &[],
[01:09:15]     |             ^^^
[01:09:15]     |             |
[01:09:15]     |             expected struct `std::vec::Vec`, found &[_; 0]
[01:09:15]     |             help: try using a conversion method: `&[].to_vec()`
[01:09:15]     |
[01:09:15]     = note: expected type `std::vec::Vec<std::string::String>`
[01:09:15]                found type `&[_; 0]`
[01:09:15]
[01:09:16] error[E0308]: mismatched types
[01:09:16]    --> tools/rls/src/build/cargo.rs:202:13
[01:09:16]     |
[01:09:16] 202 |             &[],
[01:09:16]     |             ^^^
[01:09:16]     |             |
[01:09:16]     |             expected struct `std::vec::Vec`, found &[_; 0]
[01:09:16]     |             help: try using a conversion method: `&[].to_vec()`
[01:09:16]     |
[01:09:16]     = note: expected type `std::vec::Vec<std::string::String>`
[01:09:16]                found type `&[_; 0]`
[01:09:16]
[01:09:16] error[E0308]: mismatched types
[01:09:16]    --> tools/rls/src/build/cargo.rs:206:19
[01:09:16]     |
[01:09:16] 206 |         features: &opts.features,
[01:09:16]     |                   ^^^^^^^^^^^^^^
[01:09:16]     |                   |
[01:09:16]     |                   expected struct `std::vec::Vec`, found reference
[01:09:16]     |                   help: consider removing the borrow: `opts.features`
[01:09:16]     |
[01:09:16]     = note: expected type `std::vec::Vec<std::string::String>`
[01:09:16]                found type `&std::vec::Vec<std::string::String>`
[01:09:16]
[01:09:17] error: aborting due to 8 previous errors
[01:09:17]
[01:09:17] For more information about this error, try `rustc --explain E0308`.
[01:09:17] error: Could not compile `rls`.
