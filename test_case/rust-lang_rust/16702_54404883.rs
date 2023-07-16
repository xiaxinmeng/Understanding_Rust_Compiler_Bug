 rust
cx.visit_id(ast::CRATE_NODE_ID);
cx.visit_ids(|v| {
    v.visited_outermost = true;
    visit::walk_crate(v, krate, ());
});

// since the root module isn't visited as an item (because it isn't an
// item), warn for it here.
run_lints!(cx, check_crate, krate);

visit::walk_crate(cx, krate, ());
