rust
use std::ffi::OsStr;
use std::path::PathBuf;
use std::io;

#[derive(Clone)]
pub struct DataSet {
    pub path: PathBuf,
    pub mountpoint: PathBuf,
}

pub fn get_mountpoint<P: AsRef<OsStr>>(path: P) -> Result<String, io::Error> {
    // ...
}

impl<P> TryFrom<P> for DataSet
    where P: AsRef<OsStr>
{
    type Error = io::Error;
    fn try_from(path: P) -> Result<Self, Self::Error> {
        let mountpoint: String = get_mountpoint(&path)?;
        let mountpoint = PathBuf::from(mountpoint);
        let path = path.as_ref().to_path_buf();
        Ok(Self { path, mountpoint })
    }
}
