
impl ExitStatus {
    pub fn exit_ok(self) -> Result<(), ExitStatusError>;
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
/// Subprocess status other than success.
pub struct ExitStatusError {
    wait_status: NonZeroU32 // on Unix; something else on Windows
}

impl std::error::Error for ExitStatusError { }
impl From<ExitStatusError> for ExitStatus {...}
impl ExitStatusExt for ExitStatusError { ... same as on ExitStatus ... }

impl Display for ExitStatusError {
   ... checks ExitStatusExt::signal(), ::coredump() etc.
}

impl ExitStatusError {
    fn into_io_error(self) -> std::io::Error { ... }
}

impl From<ExitStatusError> for std::io::Error {...}
