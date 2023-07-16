
use std::collections::BTreeMap;

pub type ManageHook = Box<Fn(Workspaces, &WindowSystem, Window) -> Workspaces>;
pub type LogHook = Box<FnMut(WindowManager, &WindowSystem)>;

pub trait WindowSystem {}
pub struct WindowManager;
pub struct Workspaces;
pub type Window = u64;

pub struct GeneralConfig;

pub struct InternalConfig {
    pub manage_hook: ManageHook,
    pub loghook: Option<LogHook>,
}

impl InternalConfig {
    pub fn new(manage_hook: ManageHook) -> InternalConfig {
        InternalConfig {
            manage_hook: manage_hook,
            loghook: None,
        }
    }
}

fn main() {
}
