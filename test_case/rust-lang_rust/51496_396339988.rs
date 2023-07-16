rust
macro_rules! foo { () => () } // (1)

mod m {
    // maybe somewhere in deeply nested modules
    #[macro_export] macro_rules! foo { () => () } (2)
}

macro_rules! foo { () => () } // (3), shadows (1) for the rest of the module

$crate::foo!(); // refers to (2), like from other crates, not to (1) or (3)
