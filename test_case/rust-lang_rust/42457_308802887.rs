
let frames = create_stacktrace(&|file, _| file.starts_with(env!("CARGO_MANIFEST_DIR")));
