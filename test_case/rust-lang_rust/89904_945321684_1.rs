rust
    fn named_ps(&self, f: impl Fn(&NamedParamType<'a>) -> bool) -> (Vec<&NamedParam<'a>>, bool) {
        (
            self.named.values().filter(|p| f(&p.ty)).collect(),
            if let Some(p) = &self.rest {
                f(&p.ty)
            } else {
                false
            },
        )
    }
