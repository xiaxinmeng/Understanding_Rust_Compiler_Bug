rust
    /// Returns `true` if the token the start of a item 
    pub fn can_begin_item(&self) -> bool {
        self.is_keyword(kw::Use)
        || self.is_keyword(kw::Fn)
        || self.is_keyword(kw::Extern)
        || self.is_keyword(kw::Crate)
        || self.is_keyword(kw::Mod)
        || self.is_keyword(kw::Const)
        || self.is_keyword(kw::Static)
        || self.is_keyword(kw::Trait)
        || self.is_keyword(kw::Impl)
        || self.is_keyword(kw::Mod)
        || self.is_keyword(kw::Type)
        || self.is_keyword(kw::Enum)
        || self.is_keyword(kw::Struct)
        || self.is_keyword(kw::Union)
    }

