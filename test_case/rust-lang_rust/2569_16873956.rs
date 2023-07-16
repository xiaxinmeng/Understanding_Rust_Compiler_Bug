
# inline(always)
/// Do a foo
/** Do a foo */
fn foo() {
    #[some_inner_thing]
    //! Do a foo
    /*! Do a foo */
    ...
}
