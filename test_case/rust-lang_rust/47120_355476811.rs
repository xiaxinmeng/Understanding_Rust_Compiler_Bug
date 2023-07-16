
error[E0308]: mismatched types
   --> /checkout/src/libstd/io/error.rs:582:32
    |
582 |               repr: Repr::Custom(Custom {
    |  ________________________________^
583 | |                 kind: ErrorKind::InvalidInput,
584 | |                 error: box Error {
585 | |                     repr: super::Repr::Os(code)
586 | |                 },
587 | |             })
    | |_____________^ expected struct `realstd::boxed::Box`, found struct `io::error::Custom`
    |
    = note: expected type `realstd::boxed::Box<io::error::Custom>`
               found type `io::error::Custom`
