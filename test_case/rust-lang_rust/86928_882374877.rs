
extern "C" {
    pub fn lint_me() -> A;
    //[full_tait]~^ ERROR `extern` block uses type `impl Baz`, which is not FFI-safe
}
