rust
#[macro_export]
macro_rules! m { () => () }

mod test {
    use super::*; // <- this import now fetches `m` from the root module,
                  // so `m!()` becomes ambiguous due to restrictions on interactions of modularized and legacy macros
    macro_rules! m { () => {} }

    m!();
}
