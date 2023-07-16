rust
fn run_compiler(
    at_args: &[String],
    callbacks: &mut (dyn Callbacks + Send),
    file_loader: Option<Box<dyn FileLoader + Send + Sync>>,
    make_codegen_backend: Option<
        Box<dyn FnOnce(&config::Options) -> Box<dyn CodegenBackend> + Send>,
    >,
) -> interface::Result<()> {
    let args = args::arg_expand_all(at_args);

    let Some(matches) = handle_options(&args) else { return Ok(()) };

    let sopts = config::build_session_options(&matches);
    // check dump-dep-graph and query-dep-graph here?
