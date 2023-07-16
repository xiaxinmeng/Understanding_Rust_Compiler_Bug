
macro_rules! try {
    ($expr:expr) => (match $expr {
        Result::Ok(val) => val,
        Result::Err(err) => {
            panic!("Error occured: {:?}", err)
        }
    })
}
