rust
use response::serde;
// ...
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinAddResponse {
