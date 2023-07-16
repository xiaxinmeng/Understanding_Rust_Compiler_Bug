rust

struct Error;

struct FromStringType;
struct FromStringTypeErr;

impl std::str::FromStr for FromStringType {
    type Err = FromStringTypeErr;
    fn from_str(_: &str) -> Result<Self, Self::Err> { 
        Ok(FromStringType)
    }
}

impl From<<FromStringType as std::str::FromStr>::Err> for Error {
    fn from(_: <FromStringType as std::str::FromStr>::Err) -> Self {
        Self
    }
}
