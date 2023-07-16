rust
// This works
let x = std::convert::identity(#[track_caller] || {});
// But this does not
let x = #[track_caller] || {};
