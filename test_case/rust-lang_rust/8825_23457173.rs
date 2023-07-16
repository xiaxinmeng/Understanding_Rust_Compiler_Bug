
    cx.tcx.sess.add_lint(lint::cstack,
                         callee.id,
                         callee.span,
                         fmt!("invoking non-Rust fn in fn without \
                              #[fixed_stack_segment]"));
