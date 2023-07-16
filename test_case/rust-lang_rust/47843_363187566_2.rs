
self.tcx.report(Errors::E0593 {
    borrowed_path: &borrowed_path,
    borrow_span: the_borrow_span,
    use_span: the_use_span,
});
