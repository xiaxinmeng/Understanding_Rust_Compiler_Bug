
struct MyError(String);

impl<T: std::fmt::Show> std::error::FromError<T> for MyError {
    fn from_error(err: T) -> MyError {
        MyError(format!("{}", err))
    }
}
