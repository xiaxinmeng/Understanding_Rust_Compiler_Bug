 rust
pub fn args() -> Args // Iterator<Item = String>
pub fn args_os() -> ArgsOs // Iterator<Item = OsString>

pub fn var<K: ?Sized>(key: &K) -> Result<String, VarError> where K: AsOsStr
pub fn var_os<K: ?Sized>(key: &K) -> Option<OsString> where K: AsOsStr

pub fn vars() -> Vars // Iterator<Item = (String, String)>
pub fn vars_os() -> VarsOs // Iterator<Item = (OsString, OsString)>
