
> rg '\.paths' src/librustdoc/ -A1 | grep insert
src/librustdoc/formats/cache.rs:                        self.cache.paths.insert(
src/librustdoc/formats/cache.rs-                    .insert(item.def_id.expect_real(), (self.cache.stack.clone(), item.type_()));
