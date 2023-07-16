rust
#[unstable(feature = "dir_entry_ext2")]
pub trait DirEntryExt2: Sealed {
    fn file_name_ref(&self) -> &OsStr;
    fn into_file_name(self) -> OsStr;
}
