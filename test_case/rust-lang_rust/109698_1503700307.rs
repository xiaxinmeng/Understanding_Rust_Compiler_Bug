rust
//! - When [transmuting] from `&[u8]` to `&OsStr`,
//!   - the slice may only include content from comparable `&OsStr` (see above) or be valid UTF-8
//!   - any splits of the `&OsStr` must be along char boundaries (the first byte of a UTF-8 code
//!     point sequence)
