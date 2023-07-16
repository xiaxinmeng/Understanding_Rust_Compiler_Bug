 rust
    fn read_enum<T, F>(&mut self, name: &str, f: F) -> DecodeResult<T> where
        F: FnOnce(&mut Decoder) -> DecodeResult<T>,
    {
        debug!("read_enum({})", name);
        f(self)
    }

    fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodeResult<T> where
        F: FnOnce(&mut Decoder, uint) -> DecodeResult<T>,
    {
        debug!("read_enum_variant(names={})", names);
        let name = match self.pop() {
            Json::String(s) => s,
            Json::Object(mut o) => {
                // ...
            }
        };
        let idx = match names.iter()
                             .position(|n| str::eq_slice(*n, name.as_slice())) {
            Some(idx) => idx,
            None => return Err(UnknownVariantError(name))
        };
        f(self, idx)
    }

   // ...

    // (NOT ACTUALLY CALLED?)
    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> DecodeResult<T> where
        F: FnOnce(&mut Decoder, uint) -> DecodeResult<T>,
    {
        debug!("read_enum_struct_variant(names={})", names);
        self.read_enum_variant(names, f)
    }
