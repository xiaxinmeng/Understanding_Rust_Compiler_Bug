
pub enum ErrorHandled {
    Reported,
    TooGeneric,
}

impl ErrorHandled {
    pub fn assert_reported(self) {
        match self {
            ErrorHandled::Reported => {}} 
                                     //^~ ERROR this block is empty, you might have not mean to close it
            ErrorHandled::TooGeneric => panic!(),
        }
    }
} //~ ERROR unexpected closing delimiter: `}`
