rust
match crate_a_fn {
    CrateAResult::Ok(...) => do something,
    CrateAResult::Err(...) => do something else,
}

match crate_b_fn {
    CrateBResult::Ok(...) => do something,
    CrateBResult::Err(...) => do something else,
}

...
