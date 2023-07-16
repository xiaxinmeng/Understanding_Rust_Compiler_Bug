plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:174:5
    |
174 | /     pub fn new() -> OpenOptions {
175 | |         OpenOptions {
176 | |             // generic
177 | |             read: false,
190 | |         }
191 | |     }
    | |_____^

---

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:196:5
    |
196 | /     pub fn write(&mut self, write: bool) {
197 | |         self.write = write;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:199:5
   --> library/std/src/sys/windows/fs.rs:199:5
    |
199 | /     pub fn append(&mut self, append: bool) {
200 | |         self.append = append;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:202:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:205:5
    |
205 | /     pub fn create(&mut self, create: bool) {
206 | |         self.create = create;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:208:5
   --> library/std/src/sys/windows/fs.rs:208:5
    |
208 | /     pub fn create_new(&mut self, create_new: bool) {
209 | |         self.create_new = create_new;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:212:5
   --> library/std/src/sys/windows/fs.rs:212:5
    |
212 | /     pub fn custom_flags(&mut self, flags: u32) {
213 | |         self.custom_flags = flags;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:215:5
   --> library/std/src/sys/windows/fs.rs:215:5
    |
215 | /     pub fn access_mode(&mut self, access_mode: u32) {
216 | |         self.access_mode = Some(access_mode);
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:218:5
   --> library/std/src/sys/windows/fs.rs:218:5
    |
218 | /     pub fn share_mode(&mut self, share_mode: u32) {
219 | |         self.share_mode = share_mode;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:221:5
   --> library/std/src/sys/windows/fs.rs:221:5
    |
221 | /     pub fn attributes(&mut self, attrs: u32) {
222 | |         self.attributes = attrs;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:224:5
   --> library/std/src/sys/windows/fs.rs:224:5
    |
224 | /     pub fn security_qos_flags(&mut self, flags: u32) {
225 | |         // We have to set `SECURITY_SQOS_PRESENT` here, because one of the valid flags we can
226 | |         // receive is `SECURITY_ANONYMOUS = 0x0`, which we can't check for later on.
227 | |         self.security_qos_flags = flags | c::SECURITY_SQOS_PRESENT;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:229:5
   --> library/std/src/sys/windows/fs.rs:229:5
    |
229 | /     pub fn security_attributes(&mut self, attrs: c::LPSECURITY_ATTRIBUTES) {
230 | |         self.security_attributes = attrs as usize;
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:284:5
   --> library/std/src/sys/windows/fs.rs:284:5
    |
284 | /     pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
285 | |         let path = maybe_verbatim(path)?;
287 | |             c::CreateFileW(
...   |
301 | |         }
302 | |     }
302 | |     }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:304:5
    |
304 | /     pub fn fsync(&self) -> io::Result<()> {
305 | |         cvt(unsafe { c::FlushFileBuffers(self.handle.as_raw_handle()) })?;
307 | |     }
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:309:5
    |
309 | /     pub fn datasync(&self) -> io::Result<()> {
310 | |         self.fsync()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:313:5
   --> library/std/src/sys/windows/fs.rs:313:5
    |
313 | /     pub fn truncate(&self, size: u64) -> io::Result<()> {
314 | |         let mut info = c::FILE_END_OF_FILE_INFO { EndOfFile: size as c::LARGE_INTEGER };
315 | |         let size = mem::size_of_val(&info);
316 | |         cvt(unsafe {
324 | |         Ok(())
325 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:328:5
    |
328 | /     pub fn file_attr(&self) -> io::Result<FileAttr> {
329 | |         unsafe {
330 | |             let mut info: c::BY_HANDLE_FILE_INFORMATION = mem::zeroed();
331 | |             cvt(c::GetFileInformationByHandle(self.handle.as_raw_handle(), &mut info))?;
352 | |         }
353 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:406:5
    |
406 | /     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
407 | |         self.handle.read(buf)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:410:5
   --> library/std/src/sys/windows/fs.rs:410:5
    |
410 | /     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
411 | |         self.handle.read_vectored(bufs)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:415:5
   --> library/std/src/sys/windows/fs.rs:415:5
    |
415 | /     pub fn is_read_vectored(&self) -> bool {
416 | |         self.handle.is_read_vectored()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:419:5
   --> library/std/src/sys/windows/fs.rs:419:5
    |
419 | /     pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
420 | |         self.handle.read_at(buf, offset)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:423:5
   --> library/std/src/sys/windows/fs.rs:423:5
    |
423 | /     pub fn read_buf(&self, buf: &mut ReadBuf<'_>) -> io::Result<()> {
424 | |         self.handle.read_buf(buf)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:427:5
   --> library/std/src/sys/windows/fs.rs:427:5
    |
427 | /     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
428 | |         self.handle.write(buf)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:431:5
   --> library/std/src/sys/windows/fs.rs:431:5
    |
431 | /     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
432 | |         self.handle.write_vectored(bufs)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:436:5
   --> library/std/src/sys/windows/fs.rs:436:5
    |
436 | /     pub fn is_write_vectored(&self) -> bool {
437 | |         self.handle.is_write_vectored()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:440:5
   --> library/std/src/sys/windows/fs.rs:440:5
    |
440 | /     pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
441 | |         self.handle.write_at(buf, offset)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:444:5
   --> library/std/src/sys/windows/fs.rs:444:5
    |
444 | /     pub fn flush(&self) -> io::Result<()> {
446 | |     }
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:448:5
    |
448 | /     pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
449 | |         let (whence, pos) = match pos {
450 | |             // Casting to `i64` is fine, `SetFilePointerEx` reinterprets this
451 | |             // integer as `u64`.
...   |
459 | |         Ok(newpos as u64)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:462:5
   --> library/std/src/sys/windows/fs.rs:462:5
    |
462 | /     pub fn duplicate(&self) -> io::Result<File> {
463 | |         Ok(Self { handle: self.handle.try_clone()? })
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:531:5
   --> library/std/src/sys/windows/fs.rs:531:5
    |
531 | /     pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
532 | |         let mut info = c::FILE_BASIC_INFO {
533 | |             CreationTime: 0,
534 | |             LastAccessTime: 0,
548 | |         Ok(())
549 | |     }
    | |_____^


error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:787:1
    |
787 | / impl<'a> AsHandle<'a> for &'a File {
788 | |     fn as_handle(self) -> BorrowedHandle<'a> {
789 | |         self.as_inner().as_handle()
791 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:793:1
    |
793 | / impl AsRawHandle for File {
794 | |     fn as_raw_handle(&self) -> RawHandle {
795 | |         self.as_inner().as_raw_handle()
797 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:799:1
    |
799 | / impl IntoRawHandle for File {
800 | |     fn into_raw_handle(self) -> RawHandle {
801 | |         self.into_inner().into_raw_handle()
803 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:805:1
    |
805 | / impl FromRawHandle for File {
806 | |     unsafe fn from_raw_handle(raw_handle: RawHandle) -> Self {
807 | |         Self { handle: FromInner::from_inner(FromRawHandle::from_raw_handle(raw_handle)) }
809 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:811:1
    |
811 | / impl fmt::Debug for File {
812 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
813 | |         // FIXME(#24570): add more info here (e.g., mode)
814 | |         let mut b = f.debug_struct("File");
820 | |     }
821 | | }
    | |_^

---

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:828:5
    |
828 | /     pub fn perm(&self) -> FilePermissions {
829 | |         FilePermissions { attrs: self.attributes }
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:832:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:836:5
    |
836 | /     pub fn file_type(&self) -> FileType {
837 | |         FileType::new(self.attributes, self.reparse_tag)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:840:5
   --> library/std/src/sys/windows/fs.rs:840:5
    |
840 | /     pub fn modified(&self) -> io::Result<SystemTime> {
841 | |         Ok(SystemTime::from(self.last_write_time))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:844:5
   --> library/std/src/sys/windows/fs.rs:844:5
    |
844 | /     pub fn accessed(&self) -> io::Result<SystemTime> {
845 | |         Ok(SystemTime::from(self.last_access_time))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:848:5
   --> library/std/src/sys/windows/fs.rs:848:5
    |
848 | /     pub fn created(&self) -> io::Result<SystemTime> {
849 | |         Ok(SystemTime::from(self.creation_time))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:852:5
   --> library/std/src/sys/windows/fs.rs:852:5
    |
852 | /     pub fn modified_u64(&self) -> u64 {
853 | |         to_u64(&self.last_write_time)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:856:5
   --> library/std/src/sys/windows/fs.rs:856:5
    |
856 | /     pub fn accessed_u64(&self) -> u64 {
857 | |         to_u64(&self.last_access_time)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:860:5
   --> library/std/src/sys/windows/fs.rs:860:5
    |
860 | /     pub fn created_u64(&self) -> u64 {
861 | |         to_u64(&self.creation_time)
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:864:5
   --> library/std/src/sys/windows/fs.rs:864:5
    |
864 | /     pub fn volume_serial_number(&self) -> Option<u32> {
865 | |         self.volume_serial_number
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:868:5
   --> library/std/src/sys/windows/fs.rs:868:5
    |
868 | /     pub fn number_of_links(&self) -> Option<u32> {
869 | |         self.number_of_links
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:872:5
---

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:882:5
    |
882 | /     pub fn readonly(&self) -> bool {
883 | |         self.attrs & c::FILE_ATTRIBUTE_READONLY != 0
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:886:5
   --> library/std/src/sys/windows/fs.rs:886:5
    |
886 | /     pub fn set_readonly(&mut self, readonly: bool) {
887 | |         if readonly {
888 | |             self.attrs |= c::FILE_ATTRIBUTE_READONLY;
889 | |         } else {
890 | |             self.attrs &= !c::FILE_ATTRIBUTE_READONLY;
892 | |     }
    | |_____^

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:899:5
    |
899 | /     pub fn is_dir(&self) -> bool {
900 | |         !self.is_symlink() && self.is_directory()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:902:5
   --> library/std/src/sys/windows/fs.rs:902:5
    |
902 | /     pub fn is_file(&self) -> bool {
903 | |         !self.is_symlink() && !self.is_directory()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:905:5
   --> library/std/src/sys/windows/fs.rs:905:5
    |
905 | /     pub fn is_symlink(&self) -> bool {
906 | |         self.is_reparse_point() && self.is_reparse_tag_name_surrogate()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:908:5
   --> library/std/src/sys/windows/fs.rs:908:5
    |
908 | /     pub fn is_symlink_dir(&self) -> bool {
909 | |         self.is_symlink() && self.is_directory()
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:911:5
   --> library/std/src/sys/windows/fs.rs:911:5
    |
911 | /     pub fn is_symlink_file(&self) -> bool {
912 | |         self.is_symlink() && !self.is_directory()
    | |_____^

error: associated function has missing stability attribute
  --> library/std/src/sys/windows/time.rs:74:5
  --> library/std/src/sys/windows/time.rs:74:5
   |
74 | /     pub fn now() -> SystemTime {
75 | |         unsafe {
76 | |             let mut t: SystemTime = mem::zeroed();
77 | |             c::GetSystemTimePreciseAsFileTime(&mut t.t);
79 | |         }
80 | |     }
   | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/windows/time.rs:95:5
    |
95  | /     pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
96  | |         let me = self.intervals();
97  | |         let other = other.intervals();
98  | |         if me >= other {
102 | |         }
103 | |     }
    | |_____^


error: associated function has missing stability attribute
   --> library/std/src/sys/windows/time.rs:105:5
    |
105 | /     pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
106 | |         let intervals = self.intervals().checked_add(checked_dur2intervals(other)?)?;
107 | |         Some(SystemTime::from_intervals(intervals))
    | |_____^

error: associated function has missing stability attribute
   --> library/std/src/sys/windows/time.rs:110:5
   --> library/std/src/sys/windows/time.rs:110:5
    |
110 | /     pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
111 | |         let intervals = self.intervals().checked_sub(checked_dur2intervals(other)?)?;
112 | |         Some(SystemTime::from_intervals(intervals))
    | |_____^

error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:116:1
   --> library/std/src/sys/windows/time.rs:116:1
    |
116 | / impl PartialEq for SystemTime {
117 | |     fn eq(&self, other: &SystemTime) -> bool {
118 | |         self.intervals() == other.intervals()
120 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:122:1
    |
122 | impl Eq for SystemTime {}

error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:124:1
    |
    |
124 | / impl PartialOrd for SystemTime {
125 | |     fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
126 | |         Some(self.cmp(other))
128 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:130:1
    |
130 | / impl Ord for SystemTime {
131 | |     fn cmp(&self, other: &SystemTime) -> Ordering {
132 | |         self.intervals().cmp(&other.intervals())
134 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:136:1
    |
136 | / impl fmt::Debug for SystemTime {
137 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
138 | |         f.debug_struct("SystemTime").field("intervals", &self.intervals()).finish()
140 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:142:1
    |
142 | / impl From<c::FILETIME> for SystemTime {
143 | |     fn from(t: c::FILETIME) -> SystemTime {
144 | |         SystemTime { t }
146 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:148:1
    |
148 | / impl Hash for SystemTime {
149 | |     fn hash<H: Hasher>(&self, state: &mut H) {
150 | |         self.intervals().hash(state)
152 | | }
    | |_^

error: struct has missing stability attribute
---
error: struct has missing stability attribute
  --> library/std/src/sys/windows/fs.rs:38:1
   |
38 | / pub struct FileType {
39 | |     attributes: c::DWORD,
40 | |     reparse_tag: c::DWORD,
   | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:37:10
   --> library/std/src/sys/windows/fs.rs:37:10
    |
37  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/marker.rs:391:1
    |
391 | / pub macro Copy($item:item) {
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:37:16
    |
    |
37  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/clone.rs:139:1
    |
139 | / pub macro Clone($item:item) {
139 | / pub macro Clone($item:item) {
140 | |     /* compiler built-in */
141 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:37:23
    |
37  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:37:34
    |
    |
37  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |                                    ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:37:38
    |
    |
37  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/hash/mod.rs:253:5
    |
253 | /     pub macro Hash($item:item) {
253 | /     pub macro Hash($item:item) {
254 | |         /* compiler built-in */
255 | |     }
    | |_____- in this expansion of `#[derive(Hash)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:37:44
    |
    |
37  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    |
   ::: /checkout/library/core/src/fmt/mod.rs:690:5
    |
690 | /     pub macro Debug($item:item) {
---

error: struct has missing stability attribute
  --> library/std/src/sys/windows/fs.rs:78:1
   |
78 | / pub struct FilePermissions {
79 | |     attrs: c::DWORD,
   | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:77:10
   --> library/std/src/sys/windows/fs.rs:77:10
    |
77  |   #[derive(Clone, PartialEq, Eq, Debug)]
    |
   ::: /checkout/library/core/src/clone.rs:139:1
    |
139 | / pub macro Clone($item:item) {
139 | / pub macro Clone($item:item) {
140 | |     /* compiler built-in */
141 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:77:17
    |
77  |   #[derive(Clone, PartialEq, Eq, Debug)]
    |
   ::: /checkout/library/core/src/cmp.rs:239:1
    |
239 | / pub macro PartialEq($item:item) {
239 | / pub macro PartialEq($item:item) {
240 | |     /* compiler built-in */
241 | | }
    | |_- in this expansion of `#[derive(PartialEq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:77:28
    |
    |
77  |   #[derive(Clone, PartialEq, Eq, Debug)]
    |                              ^^ in this derive macro expansion
   ::: /checkout/library/core/src/cmp.rs:305:1
    |
    |
305 | / pub macro Eq($item:item) {
307 | | }
307 | | }
    | |_- in this expansion of `#[derive(Eq)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/fs.rs:77:32
    |
    |
77  |   #[derive(Clone, PartialEq, Eq, Debug)]
    |
   ::: /checkout/library/core/src/fmt/mod.rs:690:5
    |
690 | /     pub macro Debug($item:item) {
---
error: struct has missing stability attribute
  --> library/std/src/sys/windows/time.rs:21:1
   |
21 | / pub struct SystemTime {
22 | |     t: c::FILETIME,
   | |_^

error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:20:10
---
    |
391 | / pub macro Copy($item:item) {
392 | |     /* compiler built-in */
393 | | }
    | |_- in this expansion of `#[derive(Copy)]`
error: implementation has missing stability attribute
   --> library/std/src/sys/windows/time.rs:20:16
    |
20  |   #[derive(Copy, Clone)]
