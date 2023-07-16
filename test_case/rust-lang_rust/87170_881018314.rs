plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error: duplicate diagnostic item found: `Cow`.
    |
    |
181 | / pub enum Cow<'a, B: ?Sized + 'a>
182 | | where
183 | |     B: ToOwned,
184 | | {
...   |
191 | |     Owned(#[stable(feature = "rust1", since = "1.0.0")] <B as ToOwned>::Owned),
    | |_^
    |
    |
    = note: the diagnostic item is first defined in crate `alloc`.

error: duplicate diagnostic item found: `BTreeEntry`.
   |
   |
18 | / pub enum Entry<'a, K: 'a, V: 'a> {
19 | |     /// A vacant entry.
20 | |     #[stable(feature = "rust1", since = "1.0.0")]
21 | |     Vacant(#[stable(feature = "rust1", since = "1.0.0")] VacantEntry<'a, K, V>),
...  |
25 | |     Occupied(#[stable(feature = "rust1", since = "1.0.0")] OccupiedEntry<'a, K, V>),
   | |_^
   |
   |
   = note: the diagnostic item is first defined in crate `alloc`.
error: aborting due to 2 previous errors

error: could not compile `alloc`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: duplicate diagnostic item found: `Write`.
     |
     |
1366 | / pub trait Write {
1367 | |     /// Write a buffer into this writer, returning how many bytes were written.
1368 | |     ///
1369 | |     /// This function will attempt to write the entire contents of `buf`, but
1705 | |     }
1706 | | }
     | |_^
     |
     |
     = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `DirBuilder`.
    |
196 | / pub struct DirBuilder {
196 | / pub struct DirBuilder {
197 | |     inner: fs_imp::DirBuilder,
198 | |     recursive: bool,
    | |_^
    |
    |
    = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `HashmapEntry`.
     |
     |
1833 | / pub enum Entry<'a, K: 'a, V: 'a> {
1834 | |     /// An occupied entry.
1835 | |     #[stable(feature = "rust1", since = "1.0.0")]
1836 | |     Occupied(#[stable(feature = "rust1", since = "1.0.0")] OccupiedEntry<'a, K, V>),
...    |
1840 | |     Vacant(#[stable(feature = "rust1", since = "1.0.0")] VacantEntry<'a, K, V>),
     | |_^
     |
     |
     = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `File`.
   |
92 | / pub struct File {
92 | / pub struct File {
93 | |     inner: fs_imp::File,
   | |_^
   |
   |
   = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `FileType`.
    |
    |
188 | pub struct FileType(fs_imp::FileType);
    |
    |
    = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `Read`.
    |
518 | / pub trait Read {
518 | / pub trait Read {
519 | |     /// Pull some bytes from this source into the specified buffer, returning
520 | |     /// how many bytes were read.
...   |
965 | |     }
966 | | }
    | |_^
    | |_^
    |
    = note: the diagnostic item is first defined in crate `std`.
error: aborting due to 6 previous errors

error: build failed
Build completed unsuccessfully in 0:00:34
