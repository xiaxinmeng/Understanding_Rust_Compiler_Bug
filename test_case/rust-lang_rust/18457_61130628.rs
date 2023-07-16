 rust
// path has type &Path
// Take ToCStr by value
Command::new(path.clone());  // allocates for Path (just to satisfy the by-value need)
// and allocates again for CString
// vs
// Take ToCStr by reference
Command::new(path);  // only one allocation for the CString
