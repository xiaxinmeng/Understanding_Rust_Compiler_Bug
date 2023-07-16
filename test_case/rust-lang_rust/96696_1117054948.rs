plain
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.12.0
    Checking addr2line v0.16.0
error[E0545]: `issue` must be a non-zero numeric string or "none"
     |
     |
1499 |     #[unstable(feature = "dir_entry_symlink_metadata", issue = "0")]
     |                                                                |
     |                                                                |
     |                                                                `issue` must not be "0", use "none" instead
error: associated function has missing stability attribute
    --> library/std/src/fs.rs:1500:5
     |
     |
1500 | /     pub fn symlink_metadata(&self) -> io::Result<Metadata> {
1501 | |         self.0.metadata().map(Metadata)
     | |_____^

For more information about this error, try `rustc --explain E0545`.
error: could not compile `std` due to 2 previous errors
