rust

#[derive(Debug)]
pub enum LibError {
    EmptyVec,
    DotEnvErr(Box<dyn Error>),
    LogErr(Box<dyn Error>),
    DatabaseErr(Box<dyn Error>),
}


impl Error for LibError {
    fn description(&self) -> &str {
        match self {
            EmptyVec => "empty vectors not allowed",
            DotEnvErr(e) => e.description(),
            LogErr(e) => e.description(),
            DatabaseErr(e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            EmptyVec => None,
            DotEnvErr(e) => Some(&**e),
            LogErr(e) => Some(&**e),
            DatabaseErr(e) => Some(&**e),   //<————————  here
        }
    }
}
