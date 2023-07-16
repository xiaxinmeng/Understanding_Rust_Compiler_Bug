
pub struct ScriptNew {
    pub name: String,
    pub source: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub owner_id: Option<i32>,
    pub output_id: Option<i32>,
}

impl From<<Script as Tuple>::Tuple> for ScriptNew {
    fn from(script: <ScriptEntity as Tuple>::Tuple) -> Self {
        !unimplemented!()
    }
}
