rust
pub trait TrackError<T, E> {
  fn track_error(self) -> Result<T, (E, Location)>;
}

impl<T, E> TrackError<T, E> for Result<T, E> where MyError: From<(E, Location)> {
  #[track_caller]
  fn track_error(self) -> Result<T, (E, Location)> {
    match self {
      Ok(value) => Ok(value),
      Err(error) => Err((error, Location::caller()),
    }
  }
}
