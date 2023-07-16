plain
    Checking addr2line v0.16.0
error: function has missing stability attribute
  --> library/std/src/sys_common/fs.rs:45:1
   |
45 | / pub fn try_exists(path: &Path) -> io::Result<bool> {
46 | |     match fs::metadata(path) {
47 | |         Ok(_) => Ok(true),
48 | |         Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
49 | |         Err(error) => Err(error),
51 | | }
   | |_^

error: struct has missing stability attribute
error: struct has missing stability attribute
  --> library/std/src/sys_common/rwlock.rs:39:1
   |
39 | pub struct StaticRwLockReadGuard(&'static imp::RwLock);

error: implementation has missing stability attribute
  --> library/std/src/sys_common/rwlock.rs:41:1
   |
   |
41 | / impl Drop for StaticRwLockReadGuard {
42 | |     #[inline]
43 | |     fn drop(&mut self) {
...  |
47 | |     }
48 | | }
   | |_^
---

error: module has missing stability attribute
 --> library/std/src/sys/unix/mod.rs:9:1
  |
9 | pub mod weak;

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:11:1
   |
---

error: function has missing stability attribute
  --> library/std/src/sys/unix/args.rs:13:1
   |
13 | / pub unsafe fn init(argc: isize, argv: *const *const u8) {
14 | |     imp::init(argc, argv)
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/args.rs:18:1
---

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/args.rs:26:1
   |
26 | impl !Send for Args {}

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/args.rs:27:1
   |
   |
27 | impl !Sync for Args {}

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/args.rs:29:1
   |
   |
29 | / impl fmt::Debug for Args {
30 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
31 | |         self.iter.as_slice().fmt(f)
33 | | }
   | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
  --> library/std/src/sys/unix/args.rs:35:1
   |
35 | / impl Iterator for Args {
36 | |     type Item = OsString;
37 | |     fn next(&mut self) -> Option<OsString> {
38 | |         self.iter.next()
42 | |     }
43 | | }
   | |_^


error: implementation has missing stability attribute
  --> library/std/src/sys/unix/args.rs:45:1
   |
45 | / impl ExactSizeIterator for Args {
46 | |     fn len(&self) -> usize {
47 | |         self.iter.len()
49 | | }
   | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
  --> library/std/src/sys/unix/args.rs:51:1
   |
51 | / impl DoubleEndedIterator for Args {
52 | |     fn next_back(&mut self) -> Option<OsString> {
53 | |         self.iter.next_back()
55 | | }
   | |_^

error: module has missing stability attribute
error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:15:1
   |
15 | pub mod cmath;

error: function has missing stability attribute
 --> library/std/src/sys/unix/../unix/cmath.rs:7:5
  |
  |
7 |     pub fn acos(n: f64) -> f64;

error: function has missing stability attribute
 --> library/std/src/sys/unix/../unix/cmath.rs:8:5
  |
  |
8 |     pub fn acosf(n: f32) -> f32;

error: function has missing stability attribute
 --> library/std/src/sys/unix/../unix/cmath.rs:9:5
  |
  |
9 |     pub fn asin(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:10:5
   |
   |
10 |     pub fn asinf(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:11:5
   |
   |
11 |     pub fn atan(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:12:5
   |
   |
12 |     pub fn atan2(a: f64, b: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:13:5
   |
   |
13 |     pub fn atan2f(a: f32, b: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:14:5
   |
   |
14 |     pub fn atanf(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:15:5
   |
   |
15 |     pub fn cbrt(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:16:5
   |
   |
16 |     pub fn cbrtf(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:17:5
   |
   |
17 |     pub fn cosh(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:18:5
   |
   |
18 |     pub fn coshf(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:19:5
   |
   |
19 |     pub fn expm1(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:20:5
   |
   |
20 |     pub fn expm1f(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:21:5
   |
   |
21 |     pub fn fdim(a: f64, b: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:22:5
   |
   |
22 |     pub fn fdimf(a: f32, b: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:23:5
   |
   |
23 |     pub fn hypot(x: f64, y: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:24:5
   |
   |
24 |     pub fn hypotf(x: f32, y: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:25:5
   |
   |
25 |     pub fn log1p(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:26:5
   |
   |
26 |     pub fn log1pf(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:27:5
   |
   |
27 |     pub fn sinh(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:28:5
   |
   |
28 |     pub fn sinhf(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:29:5
   |
   |
29 |     pub fn tan(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:30:5
   |
   |
30 |     pub fn tanf(n: f32) -> f32;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:31:5
   |
   |
31 |     pub fn tanh(n: f64) -> f64;

error: function has missing stability attribute
  --> library/std/src/sys/unix/../unix/cmath.rs:32:5
   |
   |
32 |     pub fn tanhf(n: f32) -> f32;

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:16:1
   |
   |
16 | pub mod env;

error: module has missing stability attribute
  --> library/std/src/sys/unix/env.rs:2:1
   |
   |
2  | / pub mod os {
3  | |     pub const FAMILY: &str = "unix";
4  | |     pub const OS: &str = "linux";
5  | |     pub const DLL_PREFIX: &str = "lib";
...  |
9  | |     pub const EXE_EXTENSION: &str = "";
   | |_^

error: constant has missing stability attribute
 --> library/std/src/sys/unix/env.rs:3:5
 --> library/std/src/sys/unix/env.rs:3:5
  |
3 |     pub const FAMILY: &str = "unix";

error: constant has missing stability attribute
 --> library/std/src/sys/unix/env.rs:4:5
  |
  |
4 |     pub const OS: &str = "linux";

error: constant has missing stability attribute
 --> library/std/src/sys/unix/env.rs:5:5
  |
  |
5 |     pub const DLL_PREFIX: &str = "lib";

error: constant has missing stability attribute
 --> library/std/src/sys/unix/env.rs:6:5
  |
  |
6 |     pub const DLL_SUFFIX: &str = ".so";

error: constant has missing stability attribute
 --> library/std/src/sys/unix/env.rs:7:5
  |
  |
7 |     pub const DLL_EXTENSION: &str = "so";

error: constant has missing stability attribute
 --> library/std/src/sys/unix/env.rs:8:5
  |
  |
8 |     pub const EXE_SUFFIX: &str = "";

error: constant has missing stability attribute
 --> library/std/src/sys/unix/env.rs:9:5
  |
  |
9 |     pub const EXE_EXTENSION: &str = "";

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:18:1
   |
---

error: struct has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:229:1
    |
229 | / pub struct ReadDir {
230 | |     inner: Arc<InnerReadDir>,
231 | |     #[cfg(not(any(
232 | |         target_os = "android",
...   |
239 | |     end_of_stream: bool,
    | |_^

error: struct has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:255:1
   --> library/std/src/sys/unix/fs.rs:255:1
    |
255 | / pub struct DirEntry {
256 | |     dir: Arc<InnerReadDir>,
257 | |     entry: dirent64_min,
258 | |     // We need to store an owned copy of the entry name on platforms that use
261 | |     name: CString,
262 | | }
    | |_^

---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:372:5
    |
372 | /     pub fn perm(&self) -> FilePermissions {
373 | |         FilePermissions { mode: (self.stat.st_mode as mode_t) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:376:5
   --> library/std/src/sys/unix/fs.rs:376:5
    |
376 | /     pub fn file_type(&self) -> FileType {
377 | |         FileType { mode: self.stat.st_mode as mode_t }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:399:5
   --> library/std/src/sys/unix/fs.rs:399:5
    |
399 | /     pub fn modified(&self) -> io::Result<SystemTime> {
400 | |         #[cfg(target_pointer_width = "32")]
401 | |         cfg_has_statx! {
402 | |             if let Some(mtime) = self.stx_mtime() {
...   |
407 | |         Ok(SystemTime::new(self.stat.st_mtime as i64, self.stat.st_mtime_nsec as i64))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:416:5
   --> library/std/src/sys/unix/fs.rs:416:5
    |
416 | /     pub fn accessed(&self) -> io::Result<SystemTime> {
417 | |         #[cfg(target_pointer_width = "32")]
418 | |         cfg_has_statx! {
419 | |             if let Some(atime) = self.stx_atime() {
...   |
424 | |         Ok(SystemTime::new(self.stat.st_atime as i64, self.stat.st_atime_nsec as i64))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:448:5
   --> library/std/src/sys/unix/fs.rs:448:5
    |
448 | /     pub fn created(&self) -> io::Result<SystemTime> {
449 | |         cfg_has_statx! {
450 | |             if let Some(ext) = &self.statx_extra_fields {
451 | |                 return if (ext.stx_mask & libc::STATX_BTIME) != 0 {
466 | |         ))
467 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:477:5
    |
477 | /     pub fn readonly(&self) -> bool {
478 | |         // check if any class (owner, group, others) has write permission
479 | |         self.mode & 0o222 == 0
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:482:5
   --> library/std/src/sys/unix/fs.rs:482:5
    |
482 | /     pub fn set_readonly(&mut self, readonly: bool) {
483 | |         if readonly {
484 | |             // remove write permission for all classes; equivalent to `chmod a-w <file>`
485 | |             self.mode &= !0o222;
489 | |         }
490 | |     }
    | |_____^

---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:497:5
    |
497 | /     pub fn is_dir(&self) -> bool {
498 | |         self.is(libc::S_IFDIR)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:500:5
   --> library/std/src/sys/unix/fs.rs:500:5
    |
500 | /     pub fn is_file(&self) -> bool {
501 | |         self.is(libc::S_IFREG)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:503:5
   --> library/std/src/sys/unix/fs.rs:503:5
    |
503 | /     pub fn is_symlink(&self) -> bool {
504 | |         self.is(libc::S_IFLNK)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:507:5
   --> library/std/src/sys/unix/fs.rs:507:5
    |
507 | /     pub fn is(&self, mode: mode_t) -> bool {
508 | |         self.mode & libc::S_IFMT == mode
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:518:1
   --> library/std/src/sys/unix/fs.rs:518:1
    |
518 | / impl fmt::Debug for ReadDir {
519 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
520 | |         // This will only be called from std::fs::ReadDir, which will add a "ReadDir()" frame.
521 | |         // Thus the result will be e g 'ReadDir("/home")'
522 | |         fmt::Debug::fmt(&*self.inner.root, f)
524 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:526:1
    |
526 | / impl Iterator for ReadDir {
527 | |     type Item = io::Result<DirEntry>;
528 | |
529 | |     #[cfg(any(
623 | |     }
624 | | }
    | |_^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:634:5
    |
634 | /     pub fn path(&self) -> PathBuf {
635 | |         self.dir.root.join(self.file_name_os_str())
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:638:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:643:5
    |
643 | /     pub fn metadata(&self) -> io::Result<FileAttr> {
644 | |         let fd = cvt(unsafe { dirfd(self.dir.dirp.0) })?;
645 | |         let name = self.name_cstr().as_ptr();
...   |
...   |
660 | |         Ok(FileAttr::from_stat64(stat))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:684:5
   --> library/std/src/sys/unix/fs.rs:684:5
    |
684 | /     pub fn file_type(&self) -> io::Result<FileType> {
685 | |         match self.entry.d_type {
686 | |             libc::DT_CHR => Ok(FileType { mode: libc::S_IFCHR }),
687 | |             libc::DT_FIFO => Ok(FileType { mode: libc::S_IFIFO }),
694 | |         }
695 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:712:5
    |
712 | /     pub fn ino(&self) -> u64 {
713 | |         self.entry.d_ino as u64
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:778:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:802:5
    |
802 | /     pub fn write(&mut self, write: bool) {
803 | |         self.write = write;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:805:5
   --> library/std/src/sys/unix/fs.rs:805:5
    |
805 | /     pub fn append(&mut self, append: bool) {
806 | |         self.append = append;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:808:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:811:5
    |
811 | /     pub fn create(&mut self, create: bool) {
812 | |         self.create = create;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:814:5
   --> library/std/src/sys/unix/fs.rs:814:5
    |
814 | /     pub fn create_new(&mut self, create_new: bool) {
815 | |         self.create_new = create_new;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:818:5
   --> library/std/src/sys/unix/fs.rs:818:5
    |
818 | /     pub fn custom_flags(&mut self, flags: i32) {
819 | |         self.custom_flags = flags;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:821:5
   --> library/std/src/sys/unix/fs.rs:821:5
    |
821 | /     pub fn mode(&mut self, mode: u32) {
822 | |         self.mode = mode as mode_t;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:862:5
   --> library/std/src/sys/unix/fs.rs:862:5
    |
862 | /     pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
863 | |         let path = cstr(path)?;
864 | |         File::open_c(&path, opts)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:867:5
   --> library/std/src/sys/unix/fs.rs:867:5
    |
867 | /     pub fn open_c(path: &CStr, opts: &OpenOptions) -> io::Result<File> {
868 | |         let flags = libc::O_CLOEXEC
869 | |             | opts.get_access_mode()?
870 | |             | opts.get_creation_mode()?
...   |
877 | |         Ok(File(unsafe { FileDesc::from_raw_fd(fd) }))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:880:5
   --> library/std/src/sys/unix/fs.rs:880:5
    |
880 | /     pub fn file_attr(&self) -> io::Result<FileAttr> {
881 | |         let fd = self.as_raw_fd();
883 | |         cfg_has_statx! {
...   |
...   |
896 | |         Ok(FileAttr::from_stat64(stat))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:899:5
   --> library/std/src/sys/unix/fs.rs:899:5
    |
899 | /     pub fn fsync(&self) -> io::Result<()> {
900 | |         cvt_r(|| unsafe { os_fsync(self.as_raw_fd()) })?;
901 | |         return Ok(());
...   |
910 | |         }
911 | |     }
    | |_____^
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:913:5
    |
913 | /     pub fn datasync(&self) -> io::Result<()> {
914 | |         cvt_r(|| unsafe { os_datasync(self.as_raw_fd()) })?;
915 | |         return Ok(());
...   |
942 | |         }
943 | |     }
    | |_____^
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:945:5
    |
945 | /     pub fn truncate(&self, size: u64) -> io::Result<()> {
946 | |         let size: off64_t =
947 | |             size.try_into().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
948 | |         cvt_r(|| unsafe { ftruncate64(self.as_raw_fd(), size) }).map(drop)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:951:5
   --> library/std/src/sys/unix/fs.rs:951:5
    |
951 | /     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
952 | |         self.0.read(buf)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:955:5
   --> library/std/src/sys/unix/fs.rs:955:5
    |
955 | /     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
956 | |         self.0.read_vectored(bufs)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:960:5
   --> library/std/src/sys/unix/fs.rs:960:5
    |
960 | /     pub fn is_read_vectored(&self) -> bool {
961 | |         self.0.is_read_vectored()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:964:5
   --> library/std/src/sys/unix/fs.rs:964:5
    |
964 | /     pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
965 | |         self.0.read_at(buf, offset)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:968:5
   --> library/std/src/sys/unix/fs.rs:968:5
    |
968 | /     pub fn read_buf(&self, buf: &mut ReadBuf<'_>) -> io::Result<()> {
969 | |         self.0.read_buf(buf)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:972:5
   --> library/std/src/sys/unix/fs.rs:972:5
    |
972 | /     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
973 | |         self.0.write(buf)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:976:5
   --> library/std/src/sys/unix/fs.rs:976:5
    |
976 | /     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
977 | |         self.0.write_vectored(bufs)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:981:5
   --> library/std/src/sys/unix/fs.rs:981:5
    |
981 | /     pub fn is_write_vectored(&self) -> bool {
982 | |         self.0.is_write_vectored()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:985:5
   --> library/std/src/sys/unix/fs.rs:985:5
    |
985 | /     pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
986 | |         self.0.write_at(buf, offset)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:989:5
---

error: associated function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:993:5
     |
993  | /     pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
994  | |         let (whence, pos) = match pos {
995  | |             // Casting to `i64` is fine, too large values will end up as
996  | |             // negative which will cause an error in `lseek64`.
1002 | |         Ok(n as u64)
1003 | |     }
     | |_____^


error: associated function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1005:5
     |
1005 | /     pub fn duplicate(&self) -> io::Result<File> {
1006 | |         self.0.duplicate().map(File)
     | |_____^

error: associated function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1009:5
    --> library/std/src/sys/unix/fs.rs:1009:5
     |
1009 | /     pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
1010 | |         cvt_r(|| unsafe { libc::fchmod(self.as_raw_fd(), perm.mode) })?;
1012 | |     }
     | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1016:5
     |
1016 | /     pub fn new() -> DirBuilder {
1017 | |         DirBuilder { mode: 0o777 }
     | |_____^

error: associated function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1020:5
    --> library/std/src/sys/unix/fs.rs:1020:5
     |
1020 | /     pub fn mkdir(&self, p: &Path) -> io::Result<()> {
1021 | |         let p = cstr(p)?;
1022 | |         cvt(unsafe { libc::mkdir(p.as_ptr(), self.mode) })?;
1024 | |     }
     | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1026:5
     |
1026 | /     pub fn set_mode(&mut self, mode: u32) {
1027 | |         self.mode = mode as mode_t;
     | |_____^

error: implementation has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1059:1
    --> library/std/src/sys/unix/fs.rs:1059:1
     |
1059 | / impl AsFd for File {
1060 | |     fn as_fd(&self) -> BorrowedFd<'_> {
1061 | |         self.0.as_fd()
1063 | | }
     | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1065:1
     |
1065 | / impl AsRawFd for File {
1066 | |     fn as_raw_fd(&self) -> RawFd {
1067 | |         self.0.as_raw_fd()
1069 | | }
     | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1071:1
     |
1071 | / impl IntoRawFd for File {
1072 | |     fn into_raw_fd(self) -> RawFd {
1073 | |         self.0.into_raw_fd()
1075 | | }
     | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1077:1
     |
1077 | / impl FromRawFd for File {
1078 | |     unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
1079 | |         Self(FromRawFd::from_raw_fd(raw_fd))
1081 | | }
     | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1083:1
     |
1083 | / impl fmt::Debug for File {
1084 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
1085 | |         #[cfg(any(target_os = "linux", target_os = "netbsd"))]
1086 | |         fn get_path(fd: c_int) -> Option<PathBuf> {
1163 | |     }
1164 | | }
     | |_^


error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1166:1
     |
1166 | / pub fn readdir(p: &Path) -> io::Result<ReadDir> {
1167 | |     let root = p.to_path_buf();
1168 | |     let p = cstr(p)?;
...    |
1188 | |     }
1189 | | }
     | |_^
     | |_^

error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1191:1
     |
1191 | / pub fn unlink(p: &Path) -> io::Result<()> {
1192 | |     let p = cstr(p)?;
1193 | |     cvt(unsafe { libc::unlink(p.as_ptr()) })?;
1195 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1197:1
     |
1197 | / pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
1198 | |     let old = cstr(old)?;
1199 | |     let new = cstr(new)?;
1200 | |     cvt(unsafe { libc::rename(old.as_ptr(), new.as_ptr()) })?;
1202 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1204:1
     |
1204 | / pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
1205 | |     let p = cstr(p)?;
1206 | |     cvt_r(|| unsafe { libc::chmod(p.as_ptr(), perm.mode) })?;
1208 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1210:1
     |
1210 | / pub fn rmdir(p: &Path) -> io::Result<()> {
1211 | |     let p = cstr(p)?;
1212 | |     cvt(unsafe { libc::rmdir(p.as_ptr()) })?;
1214 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1216:1
     |
1216 | / pub fn readlink(p: &Path) -> io::Result<PathBuf> {
1217 | |     let c_path = cstr(p)?;
1218 | |     let p = c_path.as_ptr();
...    |
1240 | |     }
1241 | | }
     | |_^
     | |_^

error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1243:1
     |
1243 | / pub fn symlink(original: &Path, link: &Path) -> io::Result<()> {
1244 | |     let original = cstr(original)?;
1245 | |     let link = cstr(link)?;
1246 | |     cvt(unsafe { libc::symlink(original.as_ptr(), link.as_ptr()) })?;
1248 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1250:1
     |
1250 | / pub fn link(original: &Path, link: &Path) -> io::Result<()> {
1251 | |     let original = cstr(original)?;
1252 | |     let link = cstr(link)?;
1253 | |     cfg_if::cfg_if! {
1280 | |     Ok(())
1281 | | }
     | |_^


error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1283:1
     |
1283 | / pub fn stat(p: &Path) -> io::Result<FileAttr> {
1284 | |     let p = cstr(p)?;
1286 | |     cfg_has_statx! {
...    |
...    |
1299 | |     Ok(FileAttr::from_stat64(stat))
     | |_^

error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1302:1
    --> library/std/src/sys/unix/fs.rs:1302:1
     |
1302 | / pub fn lstat(p: &Path) -> io::Result<FileAttr> {
1303 | |     let p = cstr(p)?;
1305 | |     cfg_has_statx! {
...    |
...    |
1318 | |     Ok(FileAttr::from_stat64(stat))
     | |_^

error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1321:1
    --> library/std/src/sys/unix/fs.rs:1321:1
     |
1321 | / pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
1322 | |     let path = CString::new(p.as_os_str().as_bytes())?;
1323 | |     let buf;
...    |
...    |
1332 | |     Ok(PathBuf::from(OsString::from_vec(buf)))
     | |_^

error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1398:1
    --> library/std/src/sys/unix/fs.rs:1398:1
     |
1398 | / pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
1399 | |     let (mut reader, reader_metadata) = open_from(from)?;
1400 | |     let max_len = u64::MAX;
1401 | |     let (mut writer, _) = open_to_and_set_permissions(to, reader_metadata)?;
1412 | |     }
1413 | | }
     | |_^


error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1526:1
     |
1526 | / pub fn chown(path: &Path, uid: u32, gid: u32) -> io::Result<()> {
1527 | |     let path = cstr(path)?;
1528 | |     cvt(unsafe { libc::chown(path.as_ptr(), uid as libc::uid_t, gid as libc::gid_t) })?;
1530 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1532:1
     |
1532 | / pub fn fchown(fd: c_int, uid: u32, gid: u32) -> io::Result<()> {
1533 | |     cvt(unsafe { libc::fchown(fd, uid as libc::uid_t, gid as libc::gid_t) })?;
1535 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1537:1
     |
1537 | / pub fn lchown(path: &Path, uid: u32, gid: u32) -> io::Result<()> {
1538 | |     let path = cstr(path)?;
1539 | |     cvt(unsafe { libc::lchown(path.as_ptr(), uid as libc::uid_t, gid as libc::gid_t) })?;
1541 | | }
     | |_^

error: function has missing stability attribute
error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1544:1
     |
1544 | / pub fn chroot(dir: &Path) -> io::Result<()> {
1545 | |     let dir = cstr(dir)?;
1546 | |     cvt(unsafe { libc::chroot(dir.as_ptr()) })?;
1548 | | }
     | |_^

error: import has missing stability attribute
---

error: function has missing stability attribute
    --> library/std/src/sys/unix/fs.rs:1739:5
     |
1739 | /     pub fn remove_dir_all(p: &Path) -> io::Result<()> {
1740 | |         remove_dir_all_modern(p)
     | |_____^

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:19:1
  --> library/std/src/sys/unix/mod.rs:19:1
   |
19 | pub mod futex;
   | ^^^^^^^^^^^^^^

error: function has missing stability attribute
  --> library/std/src/sys/unix/futex.rs:19:1
   |
19 | / pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
20 | |     use super::time::Timespec;
22 | |     use crate::sync::atomic::Ordering::Relaxed;
...  |
81 | |     }
82 | | }
82 | | }
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/futex.rs:91:1
   |
91 | / pub fn futex_wake(futex: &AtomicU32) -> bool {
92 | |     let ptr = futex as *const AtomicU32;
93 | |     let op = libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG;
94 | |     unsafe { libc::syscall(libc::SYS_futex, ptr, op, 1) > 0 }
   | |_^

error: function has missing stability attribute
   --> library/std/src/sys/unix/futex.rs:99:1
   --> library/std/src/sys/unix/futex.rs:99:1
    |
99  | / pub fn futex_wake_all(futex: &AtomicU32) {
100 | |     let ptr = futex as *const AtomicU32;
101 | |     let op = libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG;
102 | |     unsafe {
103 | |         libc::syscall(libc::SYS_futex, ptr, op, i32::MAX);
105 | | }
    | |_^

error: module has missing stability attribute
---

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/io.rs:15:5
   |
15 | /     pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
16 | |         IoSlice {
17 | |             vec: iovec { iov_base: buf.as_ptr() as *mut u8 as *mut c_void, iov_len: buf.len() },
18 | |             _p: PhantomData,
20 | |     }
   | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
  --> library/std/src/sys/unix/io.rs:23:5
   |
23 | /     pub fn advance(&mut self, n: usize) {
24 | |         if self.vec.iov_len < n {
25 | |             panic!("advancing IoSlice beyond its length");
...  |
31 | |         }
32 | |     }
   | |_____^
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/io.rs:35:5
   |
35 | /     pub fn as_slice(&self) -> &[u8] {
36 | |         unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len) }
   | |_____^

error: struct has missing stability attribute
  --> library/std/src/sys/unix/io.rs:41:1
  --> library/std/src/sys/unix/io.rs:41:1
   |
41 | / pub struct IoSliceMut<'a> {
42 | |     vec: iovec,
43 | |     _p: PhantomData<&'a mut [u8]>,
   | |_^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/io.rs:48:5
  --> library/std/src/sys/unix/io.rs:48:5
   |
48 | /     pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
49 | |         IoSliceMut {
50 | |             vec: iovec { iov_base: buf.as_mut_ptr() as *mut c_void, iov_len: buf.len() },
51 | |             _p: PhantomData,
53 | |     }
   | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
  --> library/std/src/sys/unix/io.rs:56:5
   |
56 | /     pub fn advance(&mut self, n: usize) {
57 | |         if self.vec.iov_len < n {
58 | |             panic!("advancing IoSliceMut beyond its length");
...  |
64 | |         }
65 | |     }
   | |_____^
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/io.rs:68:5
   |
68 | /     pub fn as_slice(&self) -> &[u8] {
69 | |         unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len) }
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/io.rs:73:5
  --> library/std/src/sys/unix/io.rs:73:5
   |
73 | /     pub fn as_mut_slice(&mut self) -> &mut [u8] {
74 | |         unsafe { slice::from_raw_parts_mut(self.vec.iov_base as *mut u8, self.vec.iov_len) }
   | |_____^

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:22:1
---

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:26:1
   |
26 | pub mod memchr;

error: function has missing stability attribute
  --> library/std/src/sys/unix/memchr.rs:4:1
   |
   |
4  | / pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
6  | |         libc::memchr(
6  | |         libc::memchr(
7  | |             haystack.as_ptr() as *const libc::c_void,
...  |
12 | |     if p.is_null() { None } else { Some(p.addr() - haystack.as_ptr().addr()) }
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/memchr.rs:15:1
  --> library/std/src/sys/unix/memchr.rs:15:1
   |
15 | / pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
16 | |     #[cfg(target_os = "linux")]
17 | |     fn memrchr_specific(needle: u8, haystack: &[u8]) -> Option<usize> {
18 | |         // GNU's memrchr() will - unlike memchr() - error if haystack is empty.
...  |
39 | |     memrchr_specific(needle, haystack)
   | |_^

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:28:1
  --> library/std/src/sys/unix/mod.rs:28:1
   |
28 | pub mod net;
   | ^^^^^^^^^^^^

error: import has missing stability attribute
  --> library/std/src/sys/unix/net.rs:23:1
   |
23 | pub use crate::sys::{cvt, cvt_r};

error: import has missing stability attribute
  --> library/std/src/sys/unix/net.rs:23:22
   |
   |
23 | pub use crate::sys::{cvt, cvt_r};

error: import has missing stability attribute
  --> library/std/src/sys/unix/net.rs:23:27
   |
   |
23 | pub use crate::sys::{cvt, cvt_r};

error: extern crate has missing stability attribute
  --> library/std/src/sys/unix/net.rs:26:1
   |
   |
26 | pub extern crate libc as netc;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: type alias has missing stability attribute
  --> library/std/src/sys/unix/net.rs:28:1
   |
28 | pub type wrlen_t = size_t;

error: struct has missing stability attribute
  --> library/std/src/sys/unix/net.rs:30:1
   |
---

error: function has missing stability attribute
  --> library/std/src/sys/unix/net.rs:34:1
   |
34 | / pub fn cvt_gai(err: c_int) -> io::Result<()> {
35 | |     if err == 0 {
36 | |         return Ok(());
...  |
58 | |     ))
59 | | }
   | |_^
   | |_^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/net.rs:62:5
   |
62 | /     pub fn new(addr: &SocketAddr, ty: c_int) -> io::Result<Socket> {
63 | |         let fam = match *addr {
64 | |             SocketAddr::V4(..) => libc::AF_INET,
65 | |             SocketAddr::V6(..) => libc::AF_INET6,
66 | |         };
67 | |         Socket::new_raw(fam, ty)
   | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:70:5
   --> library/std/src/sys/unix/net.rs:70:5
    |
70  | /     pub fn new_raw(fam: c_int, ty: c_int) -> io::Result<Socket> {
71  | |         unsafe {
72  | |             cfg_if::cfg_if! {
73  | |                 if #[cfg(any(
101 | |         }
102 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:105:5
    |
105 | /     pub fn new_pair(fam: c_int, ty: c_int) -> io::Result<(Socket, Socket)> {
106 | |         unsafe {
107 | |             let mut fds = [0, 0];
...   |
131 | |         }
132 | |     }
    | |_____^
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:139:5
    |
139 | /     pub fn connect_timeout(&self, addr: &SocketAddr, timeout: Duration) -> io::Result<()> {
140 | |         self.set_nonblocking(true)?;
141 | |         let r = unsafe {
142 | |             let (addrp, len) = addr.into_inner();
206 | |         }
207 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:209:5
    |
209 | /     pub fn accept(&self, storage: *mut sockaddr, len: *mut socklen_t) -> io::Result<Socket> {
210 | |         // Unfortunately the only known way right now to accept a socket and
211 | |         // atomically set the CLOEXEC flag is to use the `accept4` syscall on
212 | |         // platforms that support it. On Linux, this was added in 2.6.28,
236 | |         }
237 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:239:5
    |
239 | /     pub fn duplicate(&self) -> io::Result<Socket> {
240 | |         self.0.duplicate().map(Socket)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:250:5
   --> library/std/src/sys/unix/net.rs:250:5
    |
250 | /     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
251 | |         self.recv_with_flags(buf, 0)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:254:5
   --> library/std/src/sys/unix/net.rs:254:5
    |
254 | /     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
255 | |         self.recv_with_flags(buf, MSG_PEEK)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:258:5
   --> library/std/src/sys/unix/net.rs:258:5
    |
258 | /     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
259 | |         self.0.read_vectored(bufs)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:263:5
   --> library/std/src/sys/unix/net.rs:263:5
    |
263 | /     pub fn is_read_vectored(&self) -> bool {
264 | |         self.0.is_read_vectored()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:288:5
   --> library/std/src/sys/unix/net.rs:288:5
    |
288 | /     pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
289 | |         self.recv_from_with_flags(buf, 0)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:301:5
   --> library/std/src/sys/unix/net.rs:301:5
    |
301 | /     pub fn recv_msg(&self, msg: &mut libc::msghdr) -> io::Result<usize> {
302 | |         let n = cvt(unsafe { libc::recvmsg(self.as_raw_fd(), msg, libc::MSG_CMSG_CLOEXEC) })?;
303 | |         Ok(n as usize)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:306:5
   --> library/std/src/sys/unix/net.rs:306:5
    |
306 | /     pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
307 | |         self.recv_from_with_flags(buf, MSG_PEEK)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:310:5
   --> library/std/src/sys/unix/net.rs:310:5
    |
310 | /     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
311 | |         self.0.write(buf)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:314:5
   --> library/std/src/sys/unix/net.rs:314:5
    |
314 | /     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
315 | |         self.0.write_vectored(bufs)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:319:5
   --> library/std/src/sys/unix/net.rs:319:5
    |
319 | /     pub fn is_write_vectored(&self) -> bool {
320 | |         self.0.is_write_vectored()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:332:5
   --> library/std/src/sys/unix/net.rs:332:5
    |
332 | /     pub fn send_msg(&self, msg: &mut libc::msghdr) -> io::Result<usize> {
333 | |         let n = cvt(unsafe { libc::sendmsg(self.as_raw_fd(), msg, 0) })?;
334 | |         Ok(n as usize)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:337:5
   --> library/std/src/sys/unix/net.rs:337:5
    |
337 | /     pub fn set_timeout(&self, dur: Option<Duration>, kind: libc::c_int) -> io::Result<()> {
338 | |         let timeout = match dur {
339 | |             Some(dur) => {
340 | |                 if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
363 | |         setsockopt(self, libc::SOL_SOCKET, kind, timeout)
364 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:366:5
    |
366 | /     pub fn timeout(&self, kind: libc::c_int) -> io::Result<Option<Duration>> {
367 | |         let raw: libc::timeval = getsockopt(self, libc::SOL_SOCKET, kind)?;
368 | |         if raw.tv_sec == 0 && raw.tv_usec == 0 {
369 | |             Ok(None)
374 | |         }
375 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:377:5
    |
377 | /     pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
378 | |         let how = match how {
379 | |             Shutdown::Write => libc::SHUT_WR,
380 | |             Shutdown::Read => libc::SHUT_RD,
384 | |         Ok(())
385 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:387:5
    |
387 | /     pub fn set_linger(&self, linger: Option<Duration>) -> io::Result<()> {
388 | |         let linger = libc::linger {
389 | |             l_onoff: linger.is_some() as libc::c_int,
390 | |             l_linger: linger.unwrap_or_default().as_secs() as libc::c_int,
...   |
393 | |         setsockopt(self, libc::SOL_SOCKET, SO_LINGER, linger)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:396:5
   --> library/std/src/sys/unix/net.rs:396:5
    |
396 | /     pub fn linger(&self) -> io::Result<Option<Duration>> {
397 | |         let val: libc::linger = getsockopt(self, libc::SOL_SOCKET, SO_LINGER)?;
398 | |
399 | |         Ok((val.l_onoff != 0).then(|| Duration::from_secs(val.l_linger as u64)))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:402:5
   --> library/std/src/sys/unix/net.rs:402:5
    |
402 | /     pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
403 | |         setsockopt(self, libc::IPPROTO_TCP, libc::TCP_NODELAY, nodelay as c_int)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:406:5
   --> library/std/src/sys/unix/net.rs:406:5
    |
406 | /     pub fn nodelay(&self) -> io::Result<bool> {
407 | |         let raw: c_int = getsockopt(self, libc::IPPROTO_TCP, libc::TCP_NODELAY)?;
408 | |         Ok(raw != 0)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:412:5
   --> library/std/src/sys/unix/net.rs:412:5
    |
412 | /     pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
413 | |         setsockopt(self, libc::SOL_SOCKET, libc::SO_PASSCRED, passcred as libc::c_int)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:417:5
   --> library/std/src/sys/unix/net.rs:417:5
    |
417 | /     pub fn passcred(&self) -> io::Result<bool> {
418 | |         let passcred: libc::c_int = getsockopt(self, libc::SOL_SOCKET, libc::SO_PASSCRED)?;
419 | |         Ok(passcred != 0)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:434:5
   --> library/std/src/sys/unix/net.rs:434:5
    |
434 | /     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
435 | |         let mut nonblocking = nonblocking as libc::c_int;
436 | |         cvt(unsafe { libc::ioctl(self.as_raw_fd(), libc::FIONBIO, &mut nonblocking) }).map(drop)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:446:5
   --> library/std/src/sys/unix/net.rs:446:5
    |
446 | /     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
447 | |         let raw: c_int = getsockopt(self, libc::SOL_SOCKET, libc::SO_ERROR)?;
448 | |         if raw == 0 { Ok(None) } else { Ok(Some(io::Error::from_raw_os_error(raw as i32))) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/net.rs:452:5
   --> library/std/src/sys/unix/net.rs:452:5
    |
452 | /     pub fn as_raw(&self) -> RawFd {
453 | |         self.as_raw_fd()
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/net.rs:475:1
   --> library/std/src/sys/unix/net.rs:475:1
    |
475 | / impl AsFd for Socket {
476 | |     fn as_fd(&self) -> BorrowedFd<'_> {
477 | |         self.0.as_fd()
479 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/net.rs:481:1
    |
481 | / impl AsRawFd for Socket {
482 | |     fn as_raw_fd(&self) -> RawFd {
483 | |         self.0.as_raw_fd()
485 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/net.rs:487:1
    |
487 | / impl IntoRawFd for Socket {
488 | |     fn into_raw_fd(self) -> RawFd {
489 | |         self.0.into_raw_fd()
491 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/net.rs:493:1
    |
493 | / impl FromRawFd for Socket {
494 | |     unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
495 | |         Self(FromRawFd::from_raw_fd(raw_fd))
497 | | }
    | |_^

error: module has missing stability attribute
error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:31:1
   |
31 | pub mod os;

error: function has missing stability attribute
  --> library/std/src/sys/unix/os.rs:73:1
   |
   |
73 | / pub fn errno() -> i32 {
74 | |     unsafe { (*errno_location()) as i32 }
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/os.rs:80:1
  --> library/std/src/sys/unix/os.rs:80:1
   |
80 | / pub fn set_errno(e: i32) {
81 | |     unsafe { *errno_location() = e as c_int }
   | |_^

error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:113:1
   --> library/std/src/sys/unix/os.rs:113:1
    |
113 | / pub fn error_string(errno: i32) -> String {
114 | |     extern "C" {
115 | |         #[cfg_attr(any(target_os = "linux", target_env = "newlib"), link_name = "__xpg_strerror_r")]
116 | |         fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
129 | |     }
130 | | }
    | |_^


error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:138:1
    |
138 | / pub fn getcwd() -> io::Result<PathBuf> {
139 | |     let mut buf = Vec::with_capacity(512);
140 | |     loop {
...   |
161 | |     }
162 | | }
    | |_^
    | |_^

error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:170:1
    |
170 | / pub fn chdir(p: &path::Path) -> io::Result<()> {
171 | |     let p: &OsStr = p.as_ref();
172 | |     let p = CString::new(p.as_bytes())?;
173 | |     if unsafe { libc::chdir(p.as_ptr()) } != 0 {
176 | |     Ok(())
177 | | }
    | |_^


error: struct has missing stability attribute
   --> library/std/src/sys/unix/os.rs:179:1
    |
179 | / pub struct SplitPaths<'a> {
180 | |     iter: iter::Map<slice::Split<'a, u8, fn(&u8) -> bool>, fn(&'a [u8]) -> PathBuf>,
    | |_^

error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:183:1
   --> library/std/src/sys/unix/os.rs:183:1
    |
183 | / pub fn split_paths(unparsed: &OsStr) -> SplitPaths<'_> {
184 | |     fn bytes_to_path(b: &[u8]) -> PathBuf {
185 | |         PathBuf::from(<OsStr as OsStrExt>::from_bytes(b))
...   |
195 | |     }
196 | | }
    | |_^
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os.rs:198:1
    |
198 | / impl<'a> Iterator for SplitPaths<'a> {
199 | |     type Item = PathBuf;
200 | |     fn next(&mut self) -> Option<PathBuf> {
201 | |         self.iter.next()
205 | |     }
206 | | }
    | |_^


error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:211:1
    |
211 | / pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
212 | | where
213 | |     I: Iterator<Item = T>,
214 | |     T: AsRef<OsStr>,
...   |
228 | |     Ok(OsStringExt::from_vec(joined))
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os.rs:231:1
   --> library/std/src/sys/unix/os.rs:231:1
    |
231 | / impl fmt::Display for JoinPathsError {
232 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
233 | |         write!(f, "path segment contains separator `{}`", char::from(PATH_SEPARATOR))
235 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os.rs:237:1
    |
237 | / impl StdError for JoinPathsError {
238 | |     #[allow(deprecated)]
239 | |     fn description(&self) -> &str {
240 | |         "failed to join paths"
242 | | }
    | |_^

error: function has missing stability attribute
error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:354:1
    |
354 | / pub fn current_exe() -> io::Result<PathBuf> {
355 | |     match crate::fs::read_link("/proc/self/exe") {
356 | |         Err(ref e) if e.kind() == io::ErrorKind::NotFound => Err(io::const_io_error!(
357 | |             io::ErrorKind::Uncategorized,
361 | |     }
362 | | }
    | |_^


error: struct has missing stability attribute
   --> library/std/src/sys/unix/os.rs:474:1
    |
474 | / pub struct Env {
475 | |     iter: vec::IntoIter<(OsString, OsString)>,
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os.rs:478:1
   --> library/std/src/sys/unix/os.rs:478:1
    |
478 | impl !Send for Env {}

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os.rs:479:1
    |
    |
479 | impl !Sync for Env {}

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os.rs:481:1
    |
    |
481 | / impl Iterator for Env {
482 | |     type Item = (OsString, OsString);
483 | |     fn next(&mut self) -> Option<(OsString, OsString)> {
484 | |         self.iter.next()
488 | |     }
489 | | }
    | |_^


error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:497:1
    |
497 | / pub unsafe fn environ() -> *mut *const *const c_char {
---

error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:650:1
    |
650 | / pub fn getppid() -> u32 {
651 | |     unsafe { libc::getppid() as u32 }
    | |_^

error: function has missing stability attribute
   --> library/std/src/sys/unix/os.rs:655:1
   --> library/std/src/sys/unix/os.rs:655:1
    |
655 | / pub fn glibc_version() -> Option<(usize, usize)> {
656 | |     extern "C" {
657 | |         fn gnu_get_libc_version() -> *const libc::c_char;
...   |
664 | |     }
665 | | }
    | |_^
    | |_^

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:32:1
   |
32 | pub mod os_str;

error: struct has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:27:1
   |
   |
27 | / pub struct Slice {
28 | |     pub inner: [u8],
   | |_^

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:31:1
  --> library/std/src/sys/unix/os_str.rs:31:1
   |
31 | / impl fmt::Debug for Slice {
32 | |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
33 | |         // Writes out a valid unicode string with the correct escape sequences
...  |
46 | |     }
47 | | }
   | |_^
   | |_^

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:49:1
   |
49 | / impl fmt::Display for Slice {
50 | |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
51 | |         fmt::Display::fmt(&Utf8Lossy::from_bytes(&self.inner), formatter)
53 | | }
   | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:55:1
   |
55 | / impl fmt::Debug for Buf {
56 | |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
57 | |         fmt::Debug::fmt(self.as_slice(), formatter)
59 | | }
   | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:61:1
   |
61 | / impl fmt::Display for Buf {
62 | |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
63 | |         fmt::Display::fmt(self.as_slice(), formatter)
65 | | }
   | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:67:1
   |
67 | / impl Clone for Buf {
69 | |     fn clone(&self) -> Self {
69 | |     fn clone(&self) -> Self {
70 | |         Buf { inner: self.inner.clone() }
76 | |     }
77 | | }
   | |_^


error: associated function has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:92:5
   |
92 | /     pub fn from_string(s: String) -> Buf {
93 | |         Buf { inner: s.into_bytes() }
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:97:5
  --> library/std/src/sys/unix/os_str.rs:97:5
   |
97 | /     pub fn with_capacity(capacity: usize) -> Buf {
98 | |         Buf { inner: Vec::with_capacity(capacity) }
   | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:102:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:107:5
    |
107 | /     pub fn capacity(&self) -> usize {
108 | |         self.inner.capacity()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:112:5
   --> library/std/src/sys/unix/os_str.rs:112:5
    |
112 | /     pub fn reserve(&mut self, additional: usize) {
113 | |         self.inner.reserve(additional)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:117:5
   --> library/std/src/sys/unix/os_str.rs:117:5
    |
117 | /     pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
118 | |         self.inner.try_reserve(additional)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:122:5
   --> library/std/src/sys/unix/os_str.rs:122:5
    |
122 | /     pub fn reserve_exact(&mut self, additional: usize) {
123 | |         self.inner.reserve_exact(additional)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:127:5
   --> library/std/src/sys/unix/os_str.rs:127:5
    |
127 | /     pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
128 | |         self.inner.try_reserve_exact(additional)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:132:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:137:5
    |
137 | /     pub fn shrink_to(&mut self, min_capacity: usize) {
138 | |         self.inner.shrink_to(min_capacity)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:142:5
   --> library/std/src/sys/unix/os_str.rs:142:5
    |
142 | /     pub fn as_slice(&self) -> &Slice {
143 | |         // SAFETY: Slice just wraps [u8],
144 | |         // and &*self.inner is &[u8], therefore
145 | |         // transmuting &[u8] to &Slice is safe.
146 | |         unsafe { mem::transmute(&*self.inner) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:150:5
   --> library/std/src/sys/unix/os_str.rs:150:5
    |
150 | /     pub fn as_mut_slice(&mut self) -> &mut Slice {
151 | |         // SAFETY: Slice just wraps [u8],
152 | |         // and &mut *self.inner is &mut [u8], therefore
153 | |         // transmuting &mut [u8] to &mut Slice is safe.
154 | |         unsafe { mem::transmute(&mut *self.inner) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:157:5
   --> library/std/src/sys/unix/os_str.rs:157:5
    |
157 | /     pub fn into_string(self) -> Result<String, Buf> {
158 | |         String::from_utf8(self.inner).map_err(|p| Buf { inner: p.into_bytes() })
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:161:5
   --> library/std/src/sys/unix/os_str.rs:161:5
    |
161 | /     pub fn push_slice(&mut self, s: &Slice) {
162 | |         self.inner.extend_from_slice(&s.inner)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:166:5
   --> library/std/src/sys/unix/os_str.rs:166:5
    |
166 | /     pub fn into_box(self) -> Box<Slice> {
167 | |         unsafe { mem::transmute(self.inner.into_boxed_slice()) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:171:5
   --> library/std/src/sys/unix/os_str.rs:171:5
    |
171 | /     pub fn from_box(boxed: Box<Slice>) -> Buf {
172 | |         let inner: Box<[u8]> = unsafe { mem::transmute(boxed) };
173 | |         Buf { inner: inner.into_vec() }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:177:5
   --> library/std/src/sys/unix/os_str.rs:177:5
    |
177 | /     pub fn into_arc(&self) -> Arc<Slice> {
178 | |         self.as_slice().into_arc()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:182:5
   --> library/std/src/sys/unix/os_str.rs:182:5
    |
182 | /     pub fn into_rc(&self) -> Rc<Slice> {
183 | |         self.as_slice().into_rc()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:194:5
   --> library/std/src/sys/unix/os_str.rs:194:5
    |
194 | /     pub fn from_str(s: &str) -> &Slice {
195 | |         Slice::from_u8_slice(s.as_bytes())
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:198:5
   --> library/std/src/sys/unix/os_str.rs:198:5
    |
198 | /     pub fn to_str(&self) -> Option<&str> {
199 | |         str::from_utf8(&self.inner).ok()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:202:5
   --> library/std/src/sys/unix/os_str.rs:202:5
    |
202 | /     pub fn to_string_lossy(&self) -> Cow<'_, str> {
203 | |         String::from_utf8_lossy(&self.inner)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:206:5
   --> library/std/src/sys/unix/os_str.rs:206:5
    |
206 | /     pub fn to_owned(&self) -> Buf {
207 | |         Buf { inner: self.inner.to_vec() }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:210:5
   --> library/std/src/sys/unix/os_str.rs:210:5
    |
210 | /     pub fn clone_into(&self, buf: &mut Buf) {
211 | |         self.inner.clone_into(&mut buf.inner)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:215:5
   --> library/std/src/sys/unix/os_str.rs:215:5
    |
215 | /     pub fn into_box(&self) -> Box<Slice> {
216 | |         let boxed: Box<[u8]> = self.inner.into();
217 | |         unsafe { mem::transmute(boxed) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:220:5
   --> library/std/src/sys/unix/os_str.rs:220:5
    |
220 | /     pub fn empty_box() -> Box<Slice> {
221 | |         let boxed: Box<[u8]> = Default::default();
222 | |         unsafe { mem::transmute(boxed) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:226:5
   --> library/std/src/sys/unix/os_str.rs:226:5
    |
226 | /     pub fn into_arc(&self) -> Arc<Slice> {
227 | |         let arc: Arc<[u8]> = Arc::from(&self.inner);
228 | |         unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Slice) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:232:5
   --> library/std/src/sys/unix/os_str.rs:232:5
    |
232 | /     pub fn into_rc(&self) -> Rc<Slice> {
233 | |         let rc: Rc<[u8]> = Rc::from(&self.inner);
234 | |         unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Slice) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:238:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:248:5
    |
248 | /     pub fn to_ascii_lowercase(&self) -> Buf {
249 | |         Buf { inner: self.inner.to_ascii_lowercase() }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:253:5
   --> library/std/src/sys/unix/os_str.rs:253:5
    |
253 | /     pub fn to_ascii_uppercase(&self) -> Buf {
254 | |         Buf { inner: self.inner.to_ascii_uppercase() }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:258:5
   --> library/std/src/sys/unix/os_str.rs:258:5
    |
258 | /     pub fn is_ascii(&self) -> bool {
259 | |         self.inner.is_ascii()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:263:5
   --> library/std/src/sys/unix/os_str.rs:263:5
    |
263 | /     pub fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
264 | |         self.inner.eq_ignore_ascii_case(&other.inner)
    | |_____^

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:33:1
  --> library/std/src/sys/unix/mod.rs:33:1
   |
33 | pub mod path;
   | ^^^^^^^^^^^^^

error: function has missing stability attribute
 --> library/std/src/sys/unix/path.rs:7:1
  |
7 | / pub fn is_sep_byte(b: u8) -> bool {
8 | |     b == b'/'
  | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/path.rs:12:1
  --> library/std/src/sys/unix/path.rs:12:1
   |
12 | / pub fn is_verbatim_sep(b: u8) -> bool {
13 | |     b == b'/'
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/path.rs:17:1
  --> library/std/src/sys/unix/path.rs:17:1
   |
17 | / pub fn parse_prefix(_: &OsStr) -> Option<Prefix<'_>> {
19 | | }
   | |_^

error: constant has missing stability attribute
error: constant has missing stability attribute
  --> library/std/src/sys/unix/path.rs:21:1
   |
21 | pub const MAIN_SEP_STR: &str = "/";

error: constant has missing stability attribute
  --> library/std/src/sys/unix/path.rs:22:1
   |
   |
22 | pub const MAIN_SEP: char = '/';

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:34:1
   |
   |
34 | pub mod pipe;

error: struct has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:12:1
   |
   |
12 | pub struct AnonPipe(FileDesc);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: function has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:14:1
   |
14 | / pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
15 | |     let mut fds = [0; 2];
16 | |
17 | |     // The only known way right now to create atomically set the CLOEXEC flag is
44 | |     }
45 | | }
   | |_^


error: associated function has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:48:5
   |
48 | /     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
49 | |         self.0.read(buf)
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:52:5
  --> library/std/src/sys/unix/pipe.rs:52:5
   |
52 | /     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
53 | |         self.0.read_vectored(bufs)
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:57:5
  --> library/std/src/sys/unix/pipe.rs:57:5
   |
57 | /     pub fn is_read_vectored(&self) -> bool {
58 | |         self.0.is_read_vectored()
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:61:5
  --> library/std/src/sys/unix/pipe.rs:61:5
   |
61 | /     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
62 | |         self.0.write(buf)
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:65:5
  --> library/std/src/sys/unix/pipe.rs:65:5
   |
65 | /     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
66 | |         self.0.write_vectored(bufs)
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/pipe.rs:70:5
  --> library/std/src/sys/unix/pipe.rs:70:5
   |
70 | /     pub fn is_write_vectored(&self) -> bool {
71 | |         self.0.is_write_vectored()
   | |_____^

error: function has missing stability attribute
   --> library/std/src/sys/unix/pipe.rs:81:1
   --> library/std/src/sys/unix/pipe.rs:81:1
    |
81  | / pub fn read2(p1: AnonPipe, v1: &mut Vec<u8>, p2: AnonPipe, v2: &mut Vec<u8>) -> io::Result<()> {
82  | |     // Set both pipes into nonblocking mode as we're gonna be reading from both
83  | |     // in the `select` loop below, and we wouldn't want one to block the other!
84  | |     let p1 = p1.into_inner();
126 | |     }
127 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/sys/unix/pipe.rs:129:1
    |
129 | / impl AsRawFd for AnonPipe {
130 | |     fn as_raw_fd(&self) -> RawFd {
131 | |         self.0.as_raw_fd()
133 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/pipe.rs:135:1
    |
135 | / impl AsFd for AnonPipe {
136 | |     fn as_fd(&self) -> BorrowedFd<'_> {
137 | |         self.0.as_fd()
139 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/pipe.rs:141:1
    |
141 | / impl IntoRawFd for AnonPipe {
142 | |     fn into_raw_fd(self) -> RawFd {
143 | |         self.0.into_raw_fd()
145 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/pipe.rs:147:1
    |
147 | / impl FromRawFd for AnonPipe {
148 | |     unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
149 | |         Self(FromRawFd::from_raw_fd(raw_fd))
151 | | }
    | |_^

error: module has missing stability attribute
---

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:1:1
  |
1 | pub use self::process_common::{Command, CommandArgs, ExitCode, Stdio, StdioPipes};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:1:32
  |
  |
1 | pub use self::process_common::{Command, CommandArgs, ExitCode, Stdio, StdioPipes};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:1:41
  |
  |
1 | pub use self::process_common::{Command, CommandArgs, ExitCode, Stdio, StdioPipes};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:1:54
  |
  |
1 | pub use self::process_common::{Command, CommandArgs, ExitCode, Stdio, StdioPipes};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:1:64
  |
  |
1 | pub use self::process_common::{Command, CommandArgs, ExitCode, Stdio, StdioPipes};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:1:71
  |
  |
1 | pub use self::process_common::{Command, CommandArgs, ExitCode, Stdio, StdioPipes};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:2:1
  |
  |
2 | pub use self::process_inner::{ExitStatus, ExitStatusError, Process};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:2:31
  |
  |
2 | pub use self::process_inner::{ExitStatus, ExitStatusError, Process};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:2:43
  |
  |
2 | pub use self::process_inner::{ExitStatus, ExitStatusError, Process};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:2:60
  |
  |
2 | pub use self::process_inner::{ExitStatus, ExitStatusError, Process};

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:3:1
  |
  |
3 | pub use crate::ffi::OsString as EnvKey;

error: import has missing stability attribute
 --> library/std/src/sys/unix/process/mod.rs:4:1
  |
  |
4 | pub use crate::sys_common::process::CommandEnvs;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: struct has missing stability attribute
  --> library/std/src/sys/unix/process/process_common.rs:64:1
   |
64 | / pub struct Command {
65 | |     program: CString,
66 | |     args: Vec<CString>,
67 | |     /// Exactly what will be passed to `execvp`.
...  |
86 | |     pgroup: Option<pid_t>,
   | |_^

error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:99:1
   --> library/std/src/sys/unix/process/process_common.rs:99:1
    |
99  | / pub struct StdioPipes {
100 | |     pub stdin: Option<AnonPipe>,
101 | |     pub stdout: Option<AnonPipe>,
102 | |     pub stderr: Option<AnonPipe>,
    | |_^

error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:107:1
   --> library/std/src/sys/unix/process/process_common.rs:107:1
    |
107 | / pub struct ChildPipes {
108 | |     pub stdin: ChildStdio,
109 | |     pub stdout: ChildStdio,
110 | |     pub stderr: ChildStdio,
    | |_^

error: enum has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:113:1
   --> library/std/src/sys/unix/process/process_common.rs:113:1
    |
113 | / pub enum ChildStdio {
114 | |     Inherit,
115 | |     Explicit(c_int),
116 | |     Owned(FileDesc),
121 | |     Null,
122 | | }
    | |_^


error: enum has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:124:1
    |
124 | / pub enum Stdio {
125 | |     Inherit,
126 | |     Null,
127 | |     MakePipe,
128 | |     Fd(FileDesc),
    | |_^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:155:5
   --> library/std/src/sys/unix/process/process_common.rs:155:5
    |
155 | /     pub fn new(program: &OsStr) -> Command {
156 | |         let mut saw_nul = false;
157 | |         let program = os2c(program, &mut saw_nul);
...   |
174 | |         }
175 | |     }
    | |_____^
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:177:5
    |
177 | /     pub fn set_arg_0(&mut self, arg: &OsStr) {
178 | |         // Set a new arg0
179 | |         let arg = os2c(arg, &mut self.saw_nul);
180 | |         debug_assert!(self.argv.0.len() > 1);
181 | |         self.argv.0[0] = arg.as_ptr();
182 | |         self.args[0] = arg;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:185:5
   --> library/std/src/sys/unix/process/process_common.rs:185:5
    |
185 | /     pub fn arg(&mut self, arg: &OsStr) {
186 | |         // Overwrite the trailing null pointer in `argv` and then add a new null
187 | |         // pointer.
188 | |         let arg = os2c(arg, &mut self.saw_nul);
194 | |         self.args.push(arg);
195 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:197:5
    |
197 | /     pub fn cwd(&mut self, dir: &OsStr) {
198 | |         self.cwd = Some(os2c(dir, &mut self.saw_nul));
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:200:5
   --> library/std/src/sys/unix/process/process_common.rs:200:5
    |
200 | /     pub fn uid(&mut self, id: uid_t) {
201 | |         self.uid = Some(id);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:203:5
   --> library/std/src/sys/unix/process/process_common.rs:203:5
    |
203 | /     pub fn gid(&mut self, id: gid_t) {
204 | |         self.gid = Some(id);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:206:5
   --> library/std/src/sys/unix/process/process_common.rs:206:5
    |
206 | /     pub fn groups(&mut self, groups: &[gid_t]) {
207 | |         self.groups = Some(Box::from(groups));
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:209:5
   --> library/std/src/sys/unix/process/process_common.rs:209:5
    |
209 | /     pub fn pgroup(&mut self, pgroup: pid_t) {
210 | |         self.pgroup = Some(pgroup);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:214:5
   --> library/std/src/sys/unix/process/process_common.rs:214:5
    |
214 | /     pub fn create_pidfd(&mut self, val: bool) {
215 | |         self.create_pidfd = val;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:225:5
   --> library/std/src/sys/unix/process/process_common.rs:225:5
    |
225 | /     pub fn get_create_pidfd(&self) -> bool {
226 | |         self.create_pidfd
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:229:5
   --> library/std/src/sys/unix/process/process_common.rs:229:5
    |
229 | /     pub fn saw_nul(&self) -> bool {
230 | |         self.saw_nul
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:233:5
   --> library/std/src/sys/unix/process/process_common.rs:233:5
    |
233 | /     pub fn get_program(&self) -> &OsStr {
234 | |         OsStr::from_bytes(self.program.as_bytes())
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:237:5
   --> library/std/src/sys/unix/process/process_common.rs:237:5
    |
237 | /     pub fn get_args(&self) -> CommandArgs<'_> {
238 | |         let mut iter = self.args.iter();
239 | |         iter.next();
240 | |         CommandArgs { iter }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:243:5
   --> library/std/src/sys/unix/process/process_common.rs:243:5
    |
243 | /     pub fn get_envs(&self) -> CommandEnvs<'_> {
244 | |         self.env.iter()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:247:5
   --> library/std/src/sys/unix/process/process_common.rs:247:5
    |
247 | /     pub fn get_current_dir(&self) -> Option<&Path> {
248 | |         self.cwd.as_ref().map(|cs| Path::new(OsStr::from_bytes(cs.as_bytes())))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:251:5
   --> library/std/src/sys/unix/process/process_common.rs:251:5
    |
251 | /     pub fn get_argv(&self) -> &Vec<*const c_char> {
252 | |         &self.argv.0
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:255:5
   --> library/std/src/sys/unix/process/process_common.rs:255:5
    |
255 | /     pub fn get_program_cstr(&self) -> &CStr {
256 | |         &*self.program
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:260:5
   --> library/std/src/sys/unix/process/process_common.rs:260:5
    |
260 | /     pub fn get_cwd(&self) -> &Option<CString> {
261 | |         &self.cwd
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:264:5
   --> library/std/src/sys/unix/process/process_common.rs:264:5
    |
264 | /     pub fn get_uid(&self) -> Option<uid_t> {
265 | |         self.uid
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:268:5
   --> library/std/src/sys/unix/process/process_common.rs:268:5
    |
268 | /     pub fn get_gid(&self) -> Option<gid_t> {
269 | |         self.gid
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:272:5
   --> library/std/src/sys/unix/process/process_common.rs:272:5
    |
272 | /     pub fn get_groups(&self) -> Option<&[gid_t]> {
273 | |         self.groups.as_deref()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:276:5
   --> library/std/src/sys/unix/process/process_common.rs:276:5
    |
276 | /     pub fn get_pgroup(&self) -> Option<pid_t> {
277 | |         self.pgroup
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:280:5
   --> library/std/src/sys/unix/process/process_common.rs:280:5
    |
280 | /     pub fn get_closures(&mut self) -> &mut Vec<Box<dyn FnMut() -> io::Result<()> + Send + Sync>> {
282 | |     }
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:284:5
    |
284 | /     pub unsafe fn pre_exec(&mut self, f: Box<dyn FnMut() -> io::Result<()> + Send + Sync>) {
285 | |         self.closures.push(f);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:288:5
   --> library/std/src/sys/unix/process/process_common.rs:288:5
    |
288 | /     pub fn stdin(&mut self, stdin: Stdio) {
289 | |         self.stdin = Some(stdin);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:292:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:296:5
    |
296 | /     pub fn stderr(&mut self, stderr: Stdio) {
298 | |     }
    | |_____^

error: associated function has missing stability attribute
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:304:5
    |
304 | /     pub fn capture_env(&mut self) -> Option<CStringArray> {
305 | |         let maybe_env = self.env.capture_if_changed();
306 | |         maybe_env.map(|env| construct_envp(env, &mut self.saw_nul))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:310:5
   --> library/std/src/sys/unix/process/process_common.rs:310:5
    |
310 | /     pub fn env_saw_path(&self) -> bool {
311 | |         self.env.have_changed_path()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:315:5
   --> library/std/src/sys/unix/process/process_common.rs:315:5
    |
315 | /     pub fn program_is_path(&self) -> bool {
316 | |         self.program.to_bytes().contains(&b'/')
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:319:5
   --> library/std/src/sys/unix/process/process_common.rs:319:5
    |
319 | /     pub fn setup_io(
320 | |         &self,
321 | |         default: Stdio,
322 | |         needs_stdin: bool,
...   |
334 | |         Ok((ours, theirs))
    | |_____^

error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:346:1
   --> library/std/src/sys/unix/process/process_common.rs:346:1
    |
346 | / pub struct CStringArray {
347 | |     items: Vec<CString>,
348 | |     ptrs: Vec<*const c_char>,
    | |_^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:352:5
   --> library/std/src/sys/unix/process/process_common.rs:352:5
    |
352 | /     pub fn with_capacity(capacity: usize) -> Self {
353 | |         let mut result = CStringArray {
354 | |             items: Vec::with_capacity(capacity),
355 | |             ptrs: Vec::with_capacity(capacity + 1),
358 | |         result
359 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:360:5
    |
360 | /     pub fn push(&mut self, item: CString) {
361 | |         let l = self.ptrs.len();
362 | |         self.ptrs[l - 1] = item.as_ptr();
363 | |         self.ptrs.push(ptr::null());
364 | |         self.items.push(item);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:366:5
   --> library/std/src/sys/unix/process/process_common.rs:366:5
    |
366 | /     pub fn as_ptr(&self) -> *const *const c_char {
367 | |         self.ptrs.as_ptr()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:391:5
   --> library/std/src/sys/unix/process/process_common.rs:391:5
    |
391 | /     pub fn to_child_stdio(&self, readable: bool) -> io::Result<(ChildStdio, Option<AnonPipe>)> {
392 | |         match *self {
393 | |             Stdio::Inherit => Ok((ChildStdio::Inherit, None)),
...   |
428 | |         }
429 | |     }
    | |_____^
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:432:1
    |
432 | / impl From<AnonPipe> for Stdio {
433 | |     fn from(pipe: AnonPipe) -> Stdio {
434 | |         Stdio::Fd(pipe.into_inner())
436 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:438:1
    |
438 | / impl From<File> for Stdio {
439 | |     fn from(file: File) -> Stdio {
440 | |         Stdio::Fd(file.into_inner())
442 | | }
    | |_^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:445:5
    |
445 | /     pub fn fd(&self) -> Option<c_int> {
446 | |         match *self {
447 | |             ChildStdio::Inherit => None,
448 | |             ChildStdio::Explicit(fd) => Some(fd),
453 | |         }
454 | |     }
    | |_____^


error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:457:1
    |
457 | / impl fmt::Debug for Command {
458 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
459 | |         if self.program != self.args[0] {
460 | |             write!(f, "[{:?}] ", self.program)?;
468 | |     }
469 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:474:1
    |
474 | / impl fmt::Debug for ExitCode {
475 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
476 | |         f.debug_tuple("unix_exit_status").field(&self.0).finish()
478 | | }
    | |_^

error: associated constant has missing stability attribute
error: associated constant has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:481:5
    |
481 |     pub const SUCCESS: ExitCode = ExitCode(EXIT_SUCCESS as _);

error: associated constant has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:482:5
    |
    |
482 |     pub const FAILURE: ExitCode = ExitCode(EXIT_FAILURE as _);

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:485:5
    |
    |
485 | /     pub fn as_i32(&self) -> i32 {
486 | |         self.0 as i32
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:490:1
   --> library/std/src/sys/unix/process/process_common.rs:490:1
    |
490 | / impl From<u8> for ExitCode {
491 | |     fn from(code: u8) -> Self {
492 | |         Self(code)
494 | | }
    | |_^

error: struct has missing stability attribute
error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:496:1
    |
496 | / pub struct CommandArgs<'a> {
497 | |     iter: crate::slice::Iter<'a, CString>,
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:500:1
   --> library/std/src/sys/unix/process/process_common.rs:500:1
    |
500 | / impl<'a> Iterator for CommandArgs<'a> {
501 | |     type Item = &'a OsStr;
502 | |     fn next(&mut self) -> Option<&'a OsStr> {
503 | |         self.iter.next().map(|cs| OsStr::from_bytes(cs.as_bytes()))
507 | |     }
508 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:510:1
    |
510 | / impl<'a> ExactSizeIterator for CommandArgs<'a> {
511 | |     fn len(&self) -> usize {
512 | |         self.iter.len()
...   |
516 | |     }
517 | | }
    | |_^
    | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:519:1
    |
519 | / impl<'a> fmt::Debug for CommandArgs<'a> {
520 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
521 | |         f.debug_list().entries(self.iter.clone()).finish()
523 | | }
    | |_^

error: module has missing stability attribute
---

error: function has missing stability attribute
  --> library/std/src/sys/unix/rand.rs:4:1
   |
4  | / pub fn hashmap_random_keys() -> (u64, u64) {
5  | |     let mut v = (0, 0);
6  | |     unsafe {
7  | |         let view = slice::from_raw_parts_mut(&mut v as *mut _ as *mut u8, mem::size_of_val(&v));
10 | |     v
11 | | }
   | |_^

---

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/stack_overflow.rs:13:5
   |
13 | /     pub unsafe fn new() -> Handler {
14 | |         make_handler()
   | |_____^

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/stack_overflow.rs:22:1
  --> library/std/src/sys/unix/stack_overflow.rs:22:1
   |
22 | / impl Drop for Handler {
23 | |     fn drop(&mut self) {
24 | |         unsafe {
25 | |             drop_handler(self.data);
27 | |     }
28 | | }
   | |_^


error: function has missing stability attribute
   --> library/std/src/sys/unix/stack_overflow.rs:107:5
    |
107 | /     pub unsafe fn init() {
108 | |         let mut action: sigaction = mem::zeroed();
109 | |         for &signal in &[SIGSEGV, SIGBUS] {
110 | |             sigaction(signal, ptr::null_mut(), &mut action);
122 | |         mem::forget(handler);
123 | |     }
    | |_____^


error: function has missing stability attribute
   --> library/std/src/sys/unix/stack_overflow.rs:125:5
    |
125 | /     pub unsafe fn cleanup() {
126 | |         drop_handler(MAIN_ALTSTACK.load(Ordering::Relaxed));
    | |_____^

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:38:1
  --> library/std/src/sys/unix/mod.rs:38:1
   |
38 | pub mod stdio;
   | ^^^^^^^^^^^^^^

error: struct has missing stability attribute
 --> library/std/src/sys/unix/stdio.rs:6:1
  |
6 | pub struct Stdin(());

error: struct has missing stability attribute
 --> library/std/src/sys/unix/stdio.rs:7:1
  |
---

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/stdio.rs:16:1
   |
16 | / impl io::Read for Stdin {
17 | |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
18 | |         unsafe { ManuallyDrop::new(FileDesc::from_raw_fd(libc::STDIN_FILENO)).read(buf) }
...  |
28 | |     }
29 | | }
   | |_^
---

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/stdio.rs:37:1
   |
37 | / impl io::Write for Stdout {
38 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
39 | |         unsafe { ManuallyDrop::new(FileDesc::from_raw_fd(libc::STDOUT_FILENO)).write(buf) }
...  |
55 | |     }
56 | | }
   | |_^
---

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/stdio.rs:64:1
   |
64 | / impl io::Write for Stderr {
65 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
66 | |         unsafe { ManuallyDrop::new(FileDesc::from_raw_fd(libc::STDERR_FILENO)).write(buf) }
...  |
82 | |     }
83 | | }
   | |_^
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/stdio.rs:85:1
   |
85 | / pub fn is_ebadf(err: &io::Error) -> bool {
86 | |     err.raw_os_error() == Some(libc::EBADF as i32)
   | |_^

error: constant has missing stability attribute
  --> library/std/src/sys/unix/stdio.rs:89:1
  --> library/std/src/sys/unix/stdio.rs:89:1
   |
89 | pub const STDIN_BUF_SIZE: usize = crate::sys_common::io::DEFAULT_BUF_SIZE;

error: function has missing stability attribute
  --> library/std/src/sys/unix/stdio.rs:91:1
   |
   |
91 | / pub fn panic_output() -> Option<impl io::Write> {
92 | |     Some(Stderr::new())
   | |_^

error: module has missing stability attribute
  --> library/std/src/sys/unix/mod.rs:39:1
  --> library/std/src/sys/unix/mod.rs:39:1
   |
39 | pub mod thread;
   | ^^^^^^^^^^^^^^^

error: constant has missing stability attribute
  --> library/std/src/sys/unix/thread.rs:15:1
   |
15 | pub const DEFAULT_MIN_STACK_SIZE: usize = 2 * 1024 * 1024;

error: struct has missing stability attribute
  --> library/std/src/sys/unix/thread.rs:40:1
   |
   |
40 | / pub struct Thread {
41 | |     id: libc::pthread_t,
   | |_^

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/thread.rs:46:1
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:51:5
    |
51  | /     pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
52  | |         let p = Box::into_raw(box p);
53  | |         let mut native: libc::pthread_t = mem::zeroed();
54  | |         let mut attr: libc::pthread_attr_t = mem::zeroed();
111 | |         }
112 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:114:5
    |
114 | /     pub fn yield_now() {
115 | |         let ret = unsafe { libc::sched_yield() };
116 | |         debug_assert_eq!(ret, 0);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:120:5
   --> library/std/src/sys/unix/thread.rs:120:5
    |
120 | /     pub fn set_name(name: &CStr) {
121 | |         const PR_SET_NAME: libc::c_int = 15;
122 | |         // pthread wrapper only appeared in glibc 2.12, so we use syscall
123 | |         // directly.
132 | |         }
133 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:210:5
    |
210 | /     pub fn sleep(dur: Duration) {
211 | |         let mut secs = dur.as_secs();
212 | |         let mut nsecs = dur.subsec_nanos() as _;
...   |
232 | |         }
233 | |     }
    | |_____^
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:248:5
    |
248 | /     pub fn join(self) {
249 | |         unsafe {
250 | |             let ret = libc::pthread_join(self.id, ptr::null_mut());
251 | |             mem::forget(self);
252 | |             assert!(ret == 0, "failed to join thread: {}", io::Error::from_raw_os_error(ret));
254 | |     }
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:256:5
    |
256 | /     pub fn id(&self) -> libc::pthread_t {
257 | |         self.id
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:260:5
   --> library/std/src/sys/unix/thread.rs:260:5
    |
260 | /     pub fn into_id(self) -> libc::pthread_t {
261 | |         let id = self.id;
263 | |         id
264 | |     }
    | |_____^


error: implementation has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:267:1
    |
267 | / impl Drop for Thread {
268 | |     fn drop(&mut self) {
269 | |         let ret = unsafe { libc::pthread_detach(self.id) };
270 | |         debug_assert_eq!(ret, 0);
272 | | }
    | |_^

error: function has missing stability attribute
error: function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:274:1
    |
274 | / pub fn available_parallelism() -> io::Result<NonZeroUsize> {
275 | |     cfg_if::cfg_if! {
276 | |         if #[cfg(any(
277 | |             target_os = "android",
379 | |     }
380 | | }
    | |_^


error: module has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:490:1
    |
490 | / pub mod guard {
491 | |     use libc::{mmap, mprotect};
492 | |     use libc::{MAP_ANON, MAP_FAILED, MAP_FIXED, MAP_PRIVATE, PROT_NONE, PROT_READ, PROT_WRITE};
...   |
716 | |     }
717 | | }
    | |_^
    | |_^

error: type alias has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:502:5
    |
502 |     pub type Guard = Range<usize>;

error: function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:583:5
    |
    |
583 | /     pub unsafe fn init() -> Option<Guard> {
584 | |         let page_size = os::page_size();
585 | |         PAGE_SIZE.store(page_size, Ordering::Relaxed);
...   |
651 | |         }
652 | |     }
    | |_____^
    | |_____^

error: function has missing stability attribute
   --> library/std/src/sys/unix/thread.rs:668:5
    |
668 | /     pub unsafe fn current() -> Option<Guard> {
669 | |         let mut ret = None;
670 | |         let mut attr: libc::pthread_attr_t = crate::mem::zeroed();
671 | |         #[cfg(target_os = "freebsd")]
715 | |         ret
716 | |     }
    | |_____^

---

error: type alias has missing stability attribute
 --> library/std/src/sys/unix/thread_local_key.rs:5:1
  |
5 | pub type Key = libc::pthread_key_t;

error: function has missing stability attribute
  --> library/std/src/sys/unix/thread_local_key.rs:8:1
   |
   |
8  | / pub unsafe fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
9  | |     let mut key = 0;
10 | |     assert_eq!(libc::pthread_key_create(&mut key, mem::transmute(dtor)), 0);
12 | | }
   | |_^

error: function has missing stability attribute
error: function has missing stability attribute
  --> library/std/src/sys/unix/thread_local_key.rs:15:1
   |
15 | / pub unsafe fn set(key: Key, value: *mut u8) {
16 | |     let r = libc::pthread_setspecific(key, value as *mut _);
17 | |     debug_assert_eq!(r, 0);
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/thread_local_key.rs:21:1
  --> library/std/src/sys/unix/thread_local_key.rs:21:1
   |
21 | / pub unsafe fn get(key: Key) -> *mut u8 {
22 | |     libc::pthread_getspecific(key) as *mut u8
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/thread_local_key.rs:26:1
  --> library/std/src/sys/unix/thread_local_key.rs:26:1
   |
26 | / pub unsafe fn destroy(key: Key) {
27 | |     let r = libc::pthread_key_delete(key);
28 | |     debug_assert_eq!(r, 0);
   | |_^

error: function has missing stability attribute
  --> library/std/src/sys/unix/thread_local_key.rs:32:1
---

error: constant has missing stability attribute
 --> library/std/src/sys/unix/time.rs:7:1
  |
7 | pub const UNIX_EPOCH: SystemTime = SystemTime { t: Timespec::zero() };

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/time.rs:21:5
   |
   |
21 | /     pub fn new(tv_sec: i64, tv_nsec: i64) -> SystemTime {
22 | |         SystemTime { t: Timespec::new(tv_sec, tv_nsec) }
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/time.rs:25:5
  --> library/std/src/sys/unix/time.rs:25:5
   |
25 | /     pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
26 | |         self.t.sub_timespec(&other.t)
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/time.rs:29:5
  --> library/std/src/sys/unix/time.rs:29:5
   |
29 | /     pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
30 | |         Some(SystemTime { t: self.t.checked_add_duration(other)? })
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/time.rs:33:5
  --> library/std/src/sys/unix/time.rs:33:5
   |
33 | /     pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
34 | |         Some(SystemTime { t: self.t.checked_sub_duration(other)? })
   | |_____^

error: implementation has missing stability attribute
  --> library/std/src/sys/unix/time.rs:38:1
  --> library/std/src/sys/unix/time.rs:38:1
   |
38 | / impl From<libc::timespec> for SystemTime {
39 | |     fn from(t: libc::timespec) -> SystemTime {
40 | |         SystemTime { t: Timespec::from(t) }
42 | | }
   | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
  --> library/std/src/sys/unix/time.rs:44:1
   |
44 | / impl fmt::Debug for SystemTime {
45 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
46 | |         f.debug_struct("SystemTime")
47 | |             .field("tv_sec", &self.t.tv_sec)
50 | |     }
51 | | }
   | |_^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/time.rs:274:9
    |
274 | /         pub fn now() -> Instant {
275 | |             Instant { t: Timespec::now(libc::CLOCK_MONOTONIC) }
    | |_________^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/time.rs:278:9
   --> library/std/src/sys/unix/time.rs:278:9
    |
278 | /         pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
279 | |             self.t.sub_timespec(&other.t).ok()
    | |_________^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/time.rs:282:9
   --> library/std/src/sys/unix/time.rs:282:9
    |
282 | /         pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
283 | |             Some(Instant { t: self.t.checked_add_duration(other)? })
    | |_________^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/time.rs:286:9
   --> library/std/src/sys/unix/time.rs:286:9
    |
286 | /         pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
287 | |             Some(Instant { t: self.t.checked_sub_duration(other)? })
    | |_________^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:291:5
   --> library/std/src/sys/unix/time.rs:291:5
    |
291 | /     impl fmt::Debug for Instant {
292 | |         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
293 | |             f.debug_struct("Instant")
294 | |                 .field("tv_sec", &self.t.tv_sec)
297 | |         }
298 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/time.rs:301:9
    |
301 | /         pub fn now() -> SystemTime {
302 | |             SystemTime { t: Timespec::now(libc::CLOCK_REALTIME) }
    | |_________^

error: function has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:51:1
   --> library/std/src/sys/unix/mod.rs:51:1
    |
51  | / pub unsafe fn init(argc: isize, argv: *const *const u8) {
52  | |     // The standard streams might be closed on application startup. To prevent
53  | |     // std::io::{stdin, stdout,stderr} objects from using other unrelated file
54  | |     // resources opened later, we reopen standards streams when they are closed.
123 | |     }
124 | | }
    | |_^

---

error: function has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:137:1
    |
137 | / pub fn decode_error_kind(errno: i32) -> ErrorKind {
139 | |     match errno as libc::c_int {
139 | |     match errno as libc::c_int {
140 | |         libc::E2BIG => ArgumentListTooLong,
183 | |     }
184 | | }
    | |_^


error: trait has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:187:1
    |
187 | / pub trait IsMinusOne {
188 | |     fn is_minus_one(&self) -> bool;
    | |_^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:188:5
   --> library/std/src/sys/unix/mod.rs:188:5
    |
188 |     fn is_minus_one(&self) -> bool;

error: function has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:201:1
    |
    |
201 | / pub fn cvt<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
202 | |     if t.is_minus_one() { Err(crate::io::Error::last_os_error()) } else { Ok(t) }
    | |_^

error: function has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:205:1
   --> library/std/src/sys/unix/mod.rs:205:1
    |
205 | / pub fn cvt_r<T, F>(mut f: F) -> crate::io::Result<T>
206 | | where
207 | |     T: IsMinusOne,
208 | |     F: FnMut() -> T,
215 | |     }
216 | | }
    | |_^


error: function has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:219:1
    |
219 | / pub fn cvt_nz(error: libc::c_int) -> crate::io::Result<()> {
220 | |     if error == 0 { Ok(()) } else { Err(crate::io::Error::from_raw_os_error(error)) }
    | |_^

error: function has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:258:1
   --> library/std/src/sys/unix/mod.rs:258:1
    |
258 | / pub fn abort_internal() -> ! {
259 | |     unsafe { libc::abort() }
    | |_^

error: import has missing stability attribute
  --> library/std/src/sys/mod.rs:30:9
  --> library/std/src/sys/mod.rs:30:9
   |
30 |         pub use self::unix::*;
   |         ^^^^^^^^^^^^^^^^^^^^^^

error: struct has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:106:5
    |
106 | /     pub struct FileAttr {
107 | |         stat: stat64,
108 | |         statx_extra_fields: Option<StatxExtraFields>,
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:105:14
---

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:309:10
    |
309 |   #[derive(Clone, PartialEq, Eq, Debug)]
    |
   ::: /checkout/library/core/src/clone.rs:145:1
    |
145 | / pub macro Clone($item:item) {
145 | / pub macro Clone($item:item) {
146 | |     /* compiler built-in */
147 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:309:17
    |
309 |   #[derive(Clone, PartialEq, Eq, Debug)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:309:28
    |
    |
309 |   #[derive(Clone, PartialEq, Eq, Debug)]
    |                              ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:309:32
    |
    |
309 |   #[derive(Clone, PartialEq, Eq, Debug)]
    |
   ::: /checkout/library/core/src/fmt/mod.rs:696:5
    |
696 | /     pub macro Debug($item:item) {
---

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:314:10
    |
314 |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/marker.rs:391:1
    |
391 | / pub macro Copy($item:item) {
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:314:16
    |
    |
314 |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/clone.rs:145:1
    |
145 | / pub macro Clone($item:item) {
145 | / pub macro Clone($item:item) {
146 | |     /* compiler built-in */
147 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:314:23
    |
314 |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:314:34
    |
    |
314 |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |                                    ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:314:38
    |
    |
314 |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/hash/mod.rs:253:5
    |
253 | /     pub macro Hash($item:item) {
253 | /     pub macro Hash($item:item) {
254 | |         /* compiler built-in */
255 | |     }
    | |_____- in this expansion of `#[derive(Hash)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/fs.rs:314:44
    |
    |
314 |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/fmt/mod.rs:696:5
    |
696 | /     pub macro Debug($item:item) {
---

error: struct has missing stability attribute
  --> library/std/src/sys/unix/io.rs:8:1
   |
8  | / pub struct IoSlice<'a> {
9  | |     vec: iovec,
10 | |     _p: PhantomData<&'a [u8]>,
   | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/io.rs:6:10
---
    |
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/io.rs:6:16
    |
6   |   #[derive(Copy, Clone)]
---

error: type alias has missing stability attribute
 --> library/std/src/sys/unix/locks/futex.rs:8:1
  |
8 | pub type MovableMutex = Mutex;

error: type alias has missing stability attribute
 --> library/std/src/sys/unix/locks/futex.rs:9:1
  |
  |
9 | pub type MovableCondvar = Condvar;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: struct has missing stability attribute
  --> library/std/src/sys/unix/locks/futex.rs:11:1
   |
11 | / pub struct Mutex {
12 | |     /// 0: unlocked
13 | |     /// 1: locked, no other threads waiting
14 | |     /// 2: locked, and other threads waiting (contended)
15 | |     futex: AtomicU32,
   | |_^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex.rs:20:5
  --> library/std/src/sys/unix/locks/futex.rs:20:5
   |
20 | /     pub const fn new() -> Self {
21 | |         Self { futex: AtomicU32::new(0) }
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex.rs:25:5
  --> library/std/src/sys/unix/locks/futex.rs:25:5
   |
25 |     pub unsafe fn init(&mut self) {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex.rs:28:5
   |
28 |     pub unsafe fn destroy(&self) {}

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex.rs:31:5
   |
   |
31 | /     pub unsafe fn try_lock(&self) -> bool {
32 | |         self.futex.compare_exchange(0, 1, Acquire, Relaxed).is_ok()
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex.rs:36:5
  --> library/std/src/sys/unix/locks/futex.rs:36:5
   |
36 | /     pub unsafe fn lock(&self) {
37 | |         if self.futex.compare_exchange(0, 1, Acquire, Relaxed).is_err() {
39 | |         }
40 | |     }
   | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:92:5
    |
92  | /     pub unsafe fn unlock(&self) {
93  | |         if self.futex.swap(0, Release) == 2 {
94  | |             // We only wake up one thread. When that thread locks the mutex, it
95  | |             // will mark the mutex as contended (2) (see lock_contended above),
99  | |         }
100 | |     }
    | |_____^


error: struct has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:108:1
    |
108 | / pub struct Condvar {
109 | |     // The value of this atomic is simply incremented on every notification.
110 | |     // This is used by `.wait()` to not miss any notifications after
111 | |     // unlocking the mutex and before waiting for notifications.
112 | |     futex: AtomicU32,
    | |_^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:117:5
   --> library/std/src/sys/unix/locks/futex.rs:117:5
    |
117 | /     pub const fn new() -> Self {
118 | |         Self { futex: AtomicU32::new(0) }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:122:5
   --> library/std/src/sys/unix/locks/futex.rs:122:5
    |
122 |     pub unsafe fn init(&mut self) {}
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:125:5
    |
125 |     pub unsafe fn destroy(&self) {}

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:130:5
    |
    |
130 | /     pub unsafe fn notify_one(&self) {
131 | |         self.futex.fetch_add(1, Relaxed);
132 | |         futex_wake(&self.futex);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:135:5
   --> library/std/src/sys/unix/locks/futex.rs:135:5
    |
135 | /     pub unsafe fn notify_all(&self) {
136 | |         self.futex.fetch_add(1, Relaxed);
137 | |         futex_wake_all(&self.futex);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:140:5
   --> library/std/src/sys/unix/locks/futex.rs:140:5
    |
140 | /     pub unsafe fn wait(&self, mutex: &Mutex) {
141 | |         self.wait_optional_timeout(mutex, None);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex.rs:144:5
   --> library/std/src/sys/unix/locks/futex.rs:144:5
    |
144 | /     pub unsafe fn wait_timeout(&self, mutex: &Mutex, timeout: Duration) -> bool {
145 | |         self.wait_optional_timeout(mutex, Some(timeout))
    | |_____^

error: type alias has missing stability attribute
 --> library/std/src/sys/unix/locks/futex_rwlock.rs:7:1
 --> library/std/src/sys/unix/locks/futex_rwlock.rs:7:1
  |
7 | pub type MovableRwLock = RwLock;

error: struct has missing stability attribute
  --> library/std/src/sys/unix/locks/futex_rwlock.rs:9:1
   |
   |
9  | / pub struct RwLock {
10 | |     // The state consists of a 30-bit reader counter, a 'readers waiting' flag, and a 'writers waiting' flag.
11 | |     // Bits 0..30:
12 | |     //   0: Unlocked
20 | |     writer_notify: AtomicU32,
21 | | }
   | |_^


error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex_rwlock.rs:62:5
   |
62 | /     pub const fn new() -> Self {
63 | |         Self { state: AtomicU32::new(0), writer_notify: AtomicU32::new(0) }
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex_rwlock.rs:67:5
  --> library/std/src/sys/unix/locks/futex_rwlock.rs:67:5
   |
67 |     pub unsafe fn destroy(&self) {}

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex_rwlock.rs:70:5
   |
   |
70 | /     pub unsafe fn try_read(&self) -> bool {
71 | |         self.state
72 | |             .fetch_update(Acquire, Relaxed, |s| is_read_lockable(s).then(|| s + READ_LOCKED))
73 | |             .is_ok()
   | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/unix/locks/futex_rwlock.rs:77:5
  --> library/std/src/sys/unix/locks/futex_rwlock.rs:77:5
   |
77 | /     pub unsafe fn read(&self) {
78 | |         let state = self.state.load(Relaxed);
79 | |         if !is_read_lockable(state)
80 | |             || self
86 | |         }
87 | |     }
   | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex_rwlock.rs:90:5
    |
90  | /     pub unsafe fn read_unlock(&self) {
91  | |         let state = self.state.fetch_sub(READ_LOCKED, Release) - READ_LOCKED;
92  | |
93  | |         // It's impossible for a reader to be waiting on a read-locked RwLock,
100 | |         }
101 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex_rwlock.rs:144:5
    |
144 | /     pub unsafe fn try_write(&self) -> bool {
145 | |         self.state
146 | |             .fetch_update(Acquire, Relaxed, |s| is_unlocked(s).then(|| s + WRITE_LOCKED))
147 | |             .is_ok()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex_rwlock.rs:151:5
   --> library/std/src/sys/unix/locks/futex_rwlock.rs:151:5
    |
151 | /     pub unsafe fn write(&self) {
152 | |         if self.state.compare_exchange_weak(0, WRITE_LOCKED, Acquire, Relaxed).is_err() {
154 | |         }
155 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/locks/futex_rwlock.rs:158:5
    |
158 | /     pub unsafe fn write_unlock(&self) {
159 | |         let state = self.state.fetch_sub(WRITE_LOCKED, Release) - WRITE_LOCKED;
160 | |
161 | |         debug_assert!(is_unlocked(state));
165 | |         }
166 | |     }
    | |_____^


error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:12:9
   |
12 |         pub use futex::{Mutex, MovableMutex, Condvar, MovableCondvar};

error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:12:25
   |
   |
12 |         pub use futex::{Mutex, MovableMutex, Condvar, MovableCondvar};

error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:12:32
   |
   |
12 |         pub use futex::{Mutex, MovableMutex, Condvar, MovableCondvar};

error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:12:46
   |
   |
12 |         pub use futex::{Mutex, MovableMutex, Condvar, MovableCondvar};

error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:12:55
   |
   |
12 |         pub use futex::{Mutex, MovableMutex, Condvar, MovableCondvar};

error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:13:9
   |
   |
13 |         pub use futex_rwlock::{RwLock, MovableRwLock};

error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:13:32
   |
   |
13 |         pub use futex_rwlock::{RwLock, MovableRwLock};

error: import has missing stability attribute
  --> library/std/src/sys/unix/locks/mod.rs:13:40
   |
   |
13 |         pub use futex_rwlock::{RwLock, MovableRwLock};

error: struct has missing stability attribute
   --> library/std/src/sys/unix/os.rs:209:1
    |
    |
209 | pub struct JoinPathsError;

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os.rs:208:10
    |
---

error: struct has missing stability attribute
  --> library/std/src/sys/unix/os_str.rs:22:1
   |
22 | / pub struct Buf {
23 | |     pub inner: Vec<u8>,
   | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/os_str.rs:20:10
---
    |
253 | /     pub macro Hash($item:item) {
254 | |         /* compiler built-in */
255 | |     }
    | |_____- in this expansion of `#[derive(Hash)]`
error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:472:1
    |
472 | pub struct ExitCode(u8);
472 | pub struct ExitCode(u8);
    | ^^^^^^^^^^^^^^^^^^^^^^^^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:471:10
    |
471 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:471:21
    |
    |
471 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |                       ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:471:25
    |
    |
471 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/clone.rs:145:1
    |
145 | / pub macro Clone($item:item) {
145 | / pub macro Clone($item:item) {
146 | |     /* compiler built-in */
147 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_common.rs:471:32
    |
471 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/marker.rs:391:1
    |
391 | / pub macro Copy($item:item) {
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:39:5
    |
39  | /     pub fn spawn(
---

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:223:5
    |
223 | /     pub fn exec(&mut self, default: Stdio) -> io::Error {
224 | |         let envp = self.capture_env();
225 | |
226 | |         if self.saw_nul() {
243 | |         }
244 | |     }
    | |_____^


error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:562:1
    |
562 | / pub struct Process {
563 | |     pid: pid_t,
564 | |     status: Option<ExitStatus>,
565 | |     // On Linux, stores the pidfd created for this child.
...   |
570 | |     pidfd: Option<PidFd>,
    | |_^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:588:5
   --> library/std/src/sys/unix/process/process_unix.rs:588:5
    |
588 | /     pub fn id(&self) -> u32 {
589 | |         self.pid as u32
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:592:5
   --> library/std/src/sys/unix/process/process_unix.rs:592:5
    |
592 | /     pub fn kill(&mut self) -> io::Result<()> {
593 | |         // If we've already waited on this process then the pid can be recycled
594 | |         // and used for another process, and we probably shouldn't be killing
595 | |         // random processes, so just return an error.
603 | |         }
604 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:606:5
    |
606 | /     pub fn wait(&mut self) -> io::Result<ExitStatus> {
607 | |         use crate::sys::cvt_r;
608 | |         if let Some(status) = self.status {
609 | |             return Ok(status);
...   |
614 | |         Ok(ExitStatus::new(status))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:617:5
   --> library/std/src/sys/unix/process/process_unix.rs:617:5
    |
617 | /     pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
618 | |         if let Some(status) = self.status {
619 | |             return Ok(Some(status));
...   |
628 | |         }
629 | |     }
    | |_____^
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:639:1
    |
639 | / impl fmt::Debug for ExitStatus {
640 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
641 | |         f.debug_tuple("unix_wait_status").field(&self.0).finish()
643 | | }
    | |_^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:646:5
    |
646 | /     pub fn new(status: c_int) -> ExitStatus {
647 | |         ExitStatus(status)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:654:5
   --> library/std/src/sys/unix/process/process_unix.rs:654:5
    |
654 | /     pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
655 | |         // This assumes that WIFEXITED(status) && WEXITSTATUS==0 corresponds to status==0.  This is
656 | |         // true on all actual versions of Unix, is widely assumed, and is specified in SuS
657 | |         // https://pubs.opengroup.org/onlinepubs/9699919799/functions/wait.html .  If it is not
663 | |         }
664 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:666:5
    |
666 | /     pub fn code(&self) -> Option<i32> {
667 | |         self.exited().then(|| libc::WEXITSTATUS(self.0))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:670:5
   --> library/std/src/sys/unix/process/process_unix.rs:670:5
    |
670 | /     pub fn signal(&self) -> Option<i32> {
671 | |         libc::WIFSIGNALED(self.0).then(|| libc::WTERMSIG(self.0))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:674:5
   --> library/std/src/sys/unix/process/process_unix.rs:674:5
    |
674 | /     pub fn core_dumped(&self) -> bool {
675 | |         libc::WIFSIGNALED(self.0) && libc::WCOREDUMP(self.0)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:678:5
   --> library/std/src/sys/unix/process/process_unix.rs:678:5
    |
678 | /     pub fn stopped_signal(&self) -> Option<i32> {
679 | |         libc::WIFSTOPPED(self.0).then(|| libc::WSTOPSIG(self.0))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:682:5
   --> library/std/src/sys/unix/process/process_unix.rs:682:5
    |
682 | /     pub fn continued(&self) -> bool {
683 | |         libc::WIFCONTINUED(self.0)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:686:5
---

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:692:1
    |
692 | / impl From<c_int> for ExitStatus {
693 | |     fn from(a: c_int) -> ExitStatus {
694 | |         ExitStatus(a)
696 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:698:1
    |
698 | / impl fmt::Display for ExitStatus {
699 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
700 | |         if let Some(code) = self.code() {
701 | |             write!(f, "exit status: {code}")
715 | |     }
716 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:721:1
    |
721 | / impl Into<ExitStatus> for ExitStatusError {
722 | |     fn into(self) -> ExitStatus {
723 | |         ExitStatus(self.0.into())
725 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:727:1
    |
727 | / impl fmt::Debug for ExitStatusError {
728 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
729 | |         f.debug_tuple("unix_wait_status").field(&self.0).finish()
731 | | }
    | |_^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:734:5
    |
734 | /     pub fn code(self) -> Option<NonZeroI32> {
735 | |         ExitStatus(self.0.into()).code().map(|st| st.try_into().unwrap())
    | |_____^

error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:637:1
   --> library/std/src/sys/unix/process/process_unix.rs:637:1
    |
637 | pub struct ExitStatus(c_int);

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:636:10
    |
    |
636 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:636:21
    |
    |
636 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |                       ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:636:25
    |
    |
636 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/clone.rs:145:1
    |
145 | / pub macro Clone($item:item) {
145 | / pub macro Clone($item:item) {
146 | |     /* compiler built-in */
147 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:636:32
    |
636 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/marker.rs:391:1
    |
391 | / pub macro Copy($item:item) {
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: struct has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:719:1
    |
    |
719 | pub struct ExitStatusError(NonZero_c_int);

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:718:10
    |
    |
718 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:718:21
    |
    |
718 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |                       ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:718:25
    |
    |
718 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/clone.rs:145:1
    |
145 | / pub macro Clone($item:item) {
145 | / pub macro Clone($item:item) {
146 | |     /* compiler built-in */
147 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/process/process_unix.rs:718:32
    |
718 |   #[derive(PartialEq, Eq, Clone, Copy)]
    |
   ::: /checkout/library/core/src/marker.rs:391:1
    |
391 | / pub macro Copy($item:item) {
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: struct has missing stability attribute
  --> library/std/src/sys/unix/time.rs:10:1
   |
10 | / pub struct SystemTime {
10 | / pub struct SystemTime {
11 | |     pub(in crate::sys::unix) t: Timespec,
   | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:9:10
   --> library/std/src/sys/unix/time.rs:9:10
    |
9   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/marker.rs:391:1
    |
391 | / pub macro Copy($item:item) {
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:9:16
    |
    |
9   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/clone.rs:145:1
    |
145 | / pub macro Clone($item:item) {
145 | / pub macro Clone($item:item) {
146 | |     /* compiler built-in */
147 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:9:23
    |
9   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:9:34
    |
    |
9   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |                                    ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
    --> library/std/src/sys/unix/time.rs:9:38
     |
     |
9    |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
     |
    ::: /checkout/library/core/src/cmp.rs:1172:1
     |
1172 | / pub macro PartialOrd($item:item) {
1172 | / pub macro PartialOrd($item:item) {
1173 | |     /* compiler built-in */
1174 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:9:50
    |
    |
9   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/cmp.rs:860:1
    |
    |
860 | / pub macro Ord($item:item) {
862 | | }
862 | | }
    | |_- in this expansion of `#[derive(Ord)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:9:55
    |
    |
9   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/hash/mod.rs:253:5
    |
253 | /     pub macro Hash($item:item) {
253 | /     pub macro Hash($item:item) {
254 | |         /* compiler built-in */
255 | |     }
    | |_____- in this expansion of `#[derive(Hash)]`
error: struct has missing stability attribute
   --> library/std/src/sys/unix/time.rs:269:5
    |
269 | /     pub struct Instant {
269 | /     pub struct Instant {
270 | |         t: Timespec,
271 | |     }
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:268:14
    |
268 |       #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/marker.rs:391:1
    |
391 | / pub macro Copy($item:item) {
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:268:20
    |
    |
268 |       #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/clone.rs:145:1
    |
145 | / pub macro Clone($item:item) {
145 | / pub macro Clone($item:item) {
146 | |     /* compiler built-in */
147 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:268:27
    |
268 |       #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:268:38
    |
    |
268 |       #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |                                        ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
    --> library/std/src/sys/unix/time.rs:268:42
     |
     |
268  |       #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
     |
    ::: /checkout/library/core/src/cmp.rs:1172:1
     |
1172 | / pub macro PartialOrd($item:item) {
1172 | / pub macro PartialOrd($item:item) {
1173 | |     /* compiler built-in */
1174 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:268:54
    |
    |
268 |       #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/cmp.rs:860:1
    |
    |
860 | / pub macro Ord($item:item) {
862 | | }
862 | | }
    | |_- in this expansion of `#[derive(Ord)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/time.rs:268:59
    |
    |
268 |       #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |
   ::: /checkout/library/core/src/hash/mod.rs:253:5
    |
253 | /     pub macro Hash($item:item) {
253 | /     pub macro Hash($item:item) {
254 | |         /* compiler built-in */
255 | |     }
    | |_____- in this expansion of `#[derive(Hash)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/unix/mod.rs:192:26
    |
191 |  / macro_rules! impl_is_minus_one {
191 |  / macro_rules! impl_is_minus_one {
192 |  |     ($($t:ident)*) => ($(impl IsMinusOne for $t {
    |  |__________________________^
193 | ||         fn is_minus_one(&self) -> bool {
194 | ||             *self == -1
196 | ||     })*)
    | ||_____^
197 |  | }
    |  |_- in this expansion of `impl_is_minus_one!`
    |  |_- in this expansion of `impl_is_minus_one!`
198 | 
199 |    impl_is_minus_one! { i8 i16 i32 i64 isize }

error: could not compile `std` due to 538 previous errors
Build completed unsuccessfully in 0:01:38
