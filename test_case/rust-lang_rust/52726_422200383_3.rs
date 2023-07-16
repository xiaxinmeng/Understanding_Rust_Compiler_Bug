rust
#[macro_export(local_inner_macros)]
macro_rules! select {
    // ...

    (@codegen_finalize
        $token:ident
        $index:ident
        $selected:ident
        $handles:ident
        ()
        ()
        ()
    ) => {
        unreachable!("internal error in crossbeam-channel")
    };

    // ...
}
