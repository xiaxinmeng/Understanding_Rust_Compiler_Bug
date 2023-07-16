rust
    pub fn new(index: u32, name: Name, span: Span) -> ParamTy {
        ParamTy { idx: index, name: name, span: span }
    }
