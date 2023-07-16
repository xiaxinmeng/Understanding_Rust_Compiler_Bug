
  impl ExitStatus {
    pub fn into_result(self) -> Result<(), ExitStatusError>;
