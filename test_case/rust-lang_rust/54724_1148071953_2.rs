
$ RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/expr-proc-macro-def-site`
inner_using_outer_declarations_via_fn:+ a=10
inner_using_outer_declarations_via_fn:- a=11
inner_using_outer_declarations_via_temp:+ a=11
inner_using_outer_declarations_via_temp:- a=12
inner_using_outer_declarations_via_fn:+ a=12
inner_using_outer_declarations_via_fn:- a=13
inner_using_outer_declarations_via_temp:+ a=13
inner_using_outer_declarations_via_temp:- a=14
