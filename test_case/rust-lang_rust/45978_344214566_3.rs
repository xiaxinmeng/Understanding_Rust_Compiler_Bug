rust
if let Err(_) = file {
   return Err(Error::ConfigLoadFail);
}
let Ok(f) = file;
