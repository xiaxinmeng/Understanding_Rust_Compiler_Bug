 rust
Error(err_sp, ref msg) => panic!(cx.span_fatal(err_sp.substitute_dummy(sp), msg))
