rust
impl S {
    pub fn load() -> std::io::Result<Self> {
        let field0 = super::load_data()?;
        ...

        Ok(Self{
            field0,
            ...
        })
    }
}
