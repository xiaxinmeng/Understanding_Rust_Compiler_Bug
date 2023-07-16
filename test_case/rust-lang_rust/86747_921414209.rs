rust
        match filename {
            Some(filename) => {
                let (layer, flush_guard) = ChromeLayerBuilder::new()
                    .file(filename)
                    .include_args(should_include_function_args)
                    .build();
                (Some(layer), Box::new(flush_guard))
            }
            None => {
                struct TrivialDrop;
                impl Drop for TrivialDrop {
                    fn drop(&mut self) {
                        // Do nothing.
                    }
                }
                (None, Box::new(TrivialDrop))
            }
        }
