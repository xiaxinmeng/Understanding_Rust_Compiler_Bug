
DEBUG rustc_typeck::check::closure check_closure(
    opt_kind=Some(FnOnce),
    expected_sig=Some(ExpectedSig {
        cause_span: Some(src/lib.rs:10:5: 10:8 (#0)),
        sig: Binder(
            ([&'a i32]; c_variadic: false)->&'a i32,
            [Region(BrNamed(DefId(0:10 ~ foo[df6b]::foo::'a), 'a))]
        )
    })
)
