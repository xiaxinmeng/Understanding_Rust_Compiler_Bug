rust
#[derive(Serialize)] // where I forgot this line
#[serde(remote = "StatusCode")]
struct StatusCodeDef(#[serde(getter = "StatusCode::as_u16")] u16);

impl From<StatusCodeDef> for StatusCode {
    fn from(def: StatusCodeDef) -> Self {
        StatusCode::from_u16(def.0).unwrap()
    }
}
