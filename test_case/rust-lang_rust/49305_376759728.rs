
error[E0034]: multiple applicable items in scope
   --> src\method.rs:325:9
    |
325 |         Method::try_from(t.as_bytes())
    |         ^^^^^^^^^^^^^^^^ multiple `try_from` found
    |
    = note: candidate #1 is defined in an impl of the trait `std::convert::TryFrom` for the type `_`
note: candidate #2 is defined in the trait `convert::HttpTryFrom`
   --> src\convert.rs:22:5
    |
22  |     fn try_from(t: T) -> Result<Self, Self::Error>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: to disambiguate the method call, write `convert::HttpTryFrom::try_from(...)` instead
