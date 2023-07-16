
error[E0599]: no method named `clean` found for struct `DefPath` in the current scope
    --> src/librustdoc/clean/mod.rs:1767:76
     |
1767 |                     Visibility::Restricted(module, cx.tcx.def_path(module).clean(cx))
     |                                                                            ^^^^^ method not found in `DefPath`
