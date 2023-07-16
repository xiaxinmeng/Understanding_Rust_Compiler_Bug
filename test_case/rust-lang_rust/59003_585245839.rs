rust
pub struct SlackStatus;

impl SlackStatus {
    pub const STARTING: &'static str = "starting";
    pub const SUCCESS: &'static str = "success";
    pub const ERROR: &'static str = "error";
}
