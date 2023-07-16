rust
extern crate url;

use std::path::Path;
use url::Url;

fn main() {
	// Path.canonicalize() returns a UNC path.
	let unc_path_buf = Path::new(r"C:\Windows\System").canonicalize().expect("path");
	let unc_path = unc_path_buf.as_path();

	// Meanwhile, Url.to_file_path() returns a non-UNC path,
	// even when initialized from a UNC path.
	let file_url = Url::from_file_path(unc_path).expect("url");
	let abs_path_buf = file_url.to_file_path().expect("path");
	let abs_path = abs_path_buf.as_path();

	// unc_path and abs_path refer to the same resource,
	// and they both "start with" themselves.
	assert!(unc_path.starts_with(unc_path));
	assert!(abs_path.starts_with(abs_path));

	// But they don't "start with" each other, so these fail.
	assert!(unc_path.starts_with(abs_path));
	assert!(abs_path.starts_with(unc_path));
}
