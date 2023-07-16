rust
let p_str = unsafe { CStr::from_ptr(p) };
if let Some(project_path) = p_str.to_str().ok() {
  let project_path = PathBuf::from(project_path);
}
