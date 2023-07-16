
Compiling phf_macros v0.7.24
error[E0433]: failed to resolve: maybe a missing `extern crate token;`?
   --> /home/trangar/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_macros-0.7.24/src/lib.rs:146:23
    |
146 |         input.parse::<Token![=>]>()?;
    |                       ^^^^^^^^^^ maybe a missing `extern crate token;`?
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0433]: failed to resolve: maybe a missing `extern crate token;`?
   --> /home/trangar/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_macros-0.7.24/src/lib.rs:156:42
    |
156 |         let parsed = Punctuated::<Entry, Token![,]>::parse_terminated(input)?;
    |                                          ^^^^^^^^^ maybe a missing `extern crate token;`?
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0433]: failed to resolve: maybe a missing `extern crate token;`?
   --> /home/trangar/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_macros-0.7.24/src/lib.rs:167:40
    |
167 |         let parsed = Punctuated::<Key, Token![,]>::parse_terminated(input)?;
    |                                        ^^^^^^^^^ maybe a missing `extern crate token;`?
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0277]: the size for values of type `[Entry]` cannot be known at compilation time
   --> /home/trangar/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_macros-0.7.24/src/lib.rs:157:13
    |
157 |         let map = parsed.into_iter().collect::<Vec<_>>();
    |             ^^^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `[Entry]`
    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature

error[E0308]: mismatched types
   --> /home/trangar/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_macros-0.7.24/src/lib.rs:159:16
    |
159 |         Ok(Map(map))
    |                ^^^
    |                |
    |                expected struct `std::vec::Vec`, found slice
    |                help: try using a conversion method: `map.to_vec()`
    |
    = note: expected type `std::vec::Vec<Entry>`
               found type `[Entry]`

error[E0277]: the size for values of type `[Entry]` cannot be known at compilation time
   --> /home/trangar/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_macros-0.7.24/src/lib.rs:168:13
    |
168 |         let set = parsed
    |             ^^^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `[Entry]`
    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature

error[E0308]: mismatched types
   --> /home/trangar/.cargo/registry/src/github.com-1ecc6299db9ec823/phf_macros-0.7.24/src/lib.rs:176:16
    |
176 |         Ok(Set(set))
    |                ^^^
    |                |
    |                expected struct `std::vec::Vec`, found slice
    |                help: try using a conversion method: `set.to_vec()`
    |
    = note: expected type `std::vec::Vec<Entry>`
               found type `[Entry]`

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0277, E0308, E0433.
For more information about an error, try `rustc --explain E0277`.
error: Could not compile `phf_macros`.

To learn more, run the command again with --verbose.

