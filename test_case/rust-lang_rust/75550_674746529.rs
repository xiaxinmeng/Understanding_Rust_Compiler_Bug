rust
pub struct SourceFileName {
    // Note absence of pub
    name: FileName,
    unmapped_name: Option<FileName>
}

impl SourceFileName {
    pub fn name(&self) -> &FileName { &self.name } 
    pub fn unmapped_name(&self) -> &FileName { self.unmpped_name.as_ref().unwrap_or(self.name()) }
}
