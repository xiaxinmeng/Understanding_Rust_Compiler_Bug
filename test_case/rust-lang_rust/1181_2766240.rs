
obj FILE_writer(f: os::libc::FILE, res: option::t<@FILE_res>) {
  fn fsync() {
    fsync_()

    #[cfg(target_os = "macos")]
    fn fsync_() {  macos_os::fsync(f) }

    ...
  }
}
