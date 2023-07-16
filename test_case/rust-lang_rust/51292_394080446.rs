rs
let mapped_path = map_path(my_path).ok_or(MyError::InvalidPath(my_path))?;
