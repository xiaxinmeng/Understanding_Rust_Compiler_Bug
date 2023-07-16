 rust
/// Create a `std::vec::Vec` containing the arguments.
#[macro_export]
macro_rules! vec(
    ($($e:expr),*) => ({
        // leading _ to allow empty construction without a warning.
        let mut _temp = ::std::vec::Vec::with_capacity(vec!(c: $($e),*));
        $(_temp.push($e);)*
        _temp
    });
    ($($e:expr),+,) => (vec!($($e),+));
    // These syntactic forms are for internal use only
    (c: $e:expr $(,$e_rest:expr)*) => (1 + vec!(c: $($e_rest),*));
    (c: ) => (0);
    (c: $($e:expr),*,) => (vec!(c: $($e),*))
)
