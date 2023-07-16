rust
    pub fn new(index: u32, name: Name, _span: Span) -> ParamTy {
        ParamTy { idx: index, name: name, span: DUMMY_SP }
    }
