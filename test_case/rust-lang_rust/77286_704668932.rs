rust
pub struct RunCompiler<'a, 'b> {
    at_args: &'a [String],
    callbacks: &'b mut (dyn Callbacks + Send),
    file_loader: Option<Box<dyn FileLoader + Send + Sync>>,
    emitter: Option<Box<dyn Write + Send>>,
    make_codegen_backend:
        Option<Box<dyn FnOnce(&config::Options) -> Box<dyn CodegenBackend> + Send>>,
}

impl<'a, 'b> RunCompiler<'a, 'b> {
    pub fn new(at_args: &'a [String], callbacks: &'b mut (dyn Callbacks + Send)) -> Self {
        Self { at_args, callbacks, file_loader: None, emitter: None, make_codegen_backend: None }
    }
    pub fn set_make_codegen_backend(
        &mut self,
        make_codegen_backend: Option<
            Box<dyn FnOnce(&config::Options) -> Box<dyn CodegenBackend> + Send>,
        >,
    ) -> &mut Self {
        self.make_codegen_backend = make_codegen_backend;
        self
    }
    pub fn set_emitter(&mut self, emitter: Option<Box<dyn Write + Send>>) -> &mut Self {
        self.emitter = emitter;
        self
    }
    pub fn set_file_loader(
        &mut self,
        file_loader: Option<Box<dyn FileLoader + Send + Sync>>,
    ) -> &mut Self {
        self.file_loader = file_loader;
        self
    }
    pub fn run(&mut self) -> interface::Result<()> {
        run_compiler(
            self.at_args,
            self.callbacks,
           /* These fields are behind mutable reference hence can't be moved. */
            self.file_loader,  
            self.emitter,
            self.make_codegen_backend,
        )
    }
}
