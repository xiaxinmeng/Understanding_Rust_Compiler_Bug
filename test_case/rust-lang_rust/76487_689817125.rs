rust
let is_file = path.symlink_metadata()?.file_type().is_file();
let is_dir = path.symlink_metadata()?.file_type().is_dir();
let is_symlink = path.symlink_metadata()?.file_type().is_symlink();
