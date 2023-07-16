rust
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[…]
error[E0034]: multiple applicable items in scope
   --> /home/simon/.cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:132:9
    |
132 |         Operator::try_from(*self).is_ok()
    |         ^^^^^^^^^^^^^^^^^^ multiple `try_from` found
    |
    = note: candidate #1 is defined in an impl of the trait `std::convert::TryFrom` for the type `_`
    = help: to disambiguate the method call, write `std::convert::TryFrom::try_from(...)` instead
note: candidate #2 is defined in the trait `css::token::MyTryFrom`
   --> /home/simon/.cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:29:5
    |
29  |     fn try_from(value: T) -> Result<Self, Self::Error>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: to disambiguate the method call, write `css::token::MyTryFrom::try_from(...)` instead

