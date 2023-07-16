rust
    pub fn expand_crate(&mut self, mut krate: ast::Crate) -> ast::Crate {
        let mut module = ModuleData {
            mod_path: vec![Ident::from_str(&self.cx.ecfg.crate_name)],
            directory: match self.cx.source_map().span_to_unmapped_path(krate.span) {
                FileName::Real(path) => path,
                other => PathBuf::from(other.to_string()),
            },
        };
        log::debug!("expand_crate, module #0 = {:?}", module);
        module.directory.pop();
        log::debug!("expand_crate, module #1 = {:?}", module);
        self.cx.root_path = module.directory.clone();
        self.cx.current_expansion.module = Rc::new(module);
