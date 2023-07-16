plain
    Checking hashbrown v0.12.3
    Checking object v0.26.2
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error: expected one of `!` or `::`, found doc comment `//! Filesystem manipulation operations.`
  |
  |
1 | b//! Filesystem manipulation operations.
  |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected one of `!` or `::`
error[E0432]: unresolved imports `crate::fs::OpenOptions`, `crate::fs::Permissions`
 --> library/std/src/os/unix/fs.rs:8:23
  |
  |
8 | use crate::fs::{self, OpenOptions, Permissions};
  |                       ^^^^^^^^^^^  ^^^^^^^^^^^ no `Permissions` in `fs`
  |                       no `OpenOptions` in `fs`

error[E0432]: unresolved import `crate::fs::Metadata`
 --> library/std/src/os/linux/fs.rs:7:5
 --> library/std/src/os/linux/fs.rs:7:5
  |
7 | use crate::fs::Metadata;
  |     ^^^^^^^^^^^^^^^^^^^ no `Metadata` in `fs`
error[E0432]: unresolved import `crate::fs::File`
    --> library/std/src/sys/unix/fs.rs:1466:9
     |
1466 |     use crate::fs::File;
---
     |
1493 |     use crate::fs::OpenOptions;
     |         ^^^^^^^^^^^^^^^^^^^^^^ no `OpenOptions` in `fs`

error[E0432]: unresolved imports `crate::fs::File`, `crate::fs::Metadata`
  --> library/std/src/sys/unix/kernel_copy.rs:48:17
   |
48 | use crate::fs::{File, Metadata};
   |                 ^^^^  ^^^^^^^^ no `Metadata` in `fs`
   |                 no `File` in `fs`

error[E0432]: unresolved import `crate::fs::File`
  --> library/std/src/sys/unix/rand.rs:26:9
  --> library/std/src/sys/unix/rand.rs:26:9
   |
26 |     use crate::fs::File;
   |         ^^^^^^^^^^^^^^^ no `File` in `fs`

error[E0432]: unresolved imports `crate::fs::try_exists`, `crate::fs::File`
    |
    |
396 |     use crate::fs::{try_exists, File};
    |                     ^^^^^^^^^^  ^^^^ no `File` in `fs`
    |                     no `try_exists` in `fs`


error[E0432]: unresolved import `super::mystd::fs::File`
 --> library/std/src/../../backtrace/src/symbolize/gimli/mmap_unix.rs:1:5
1 | use super::mystd::fs::File;
  |     ^^^^^^^^^^^^^^^^^^^^^^ no `File` in `fs`

error[E0432]: unresolved import `mystd::fs::File`
---
    |
664 | impl MetadataExt for fs::Metadata {
    |                          ^^^^^^^^ not found in `fs`
    |
help: there is an enum variant `crate::sys::kernel_copy::FdMeta::Metadata` and 1 other; try using the variant's enum
    |
664 | impl MetadataExt for crate::sys::kernel_copy::FdMeta {
664 | impl MetadataExt for object::SectionKind {
    |                      ~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `FileType` in module `fs`
---

error[E0412]: cannot find type `DirEntry` in module `fs`
   --> library/std/src/os/unix/fs.rs:841:26
    |
841 | impl DirEntryExt for fs::DirEntry {
    |                          ^^^^^^^^ not found in `fs`
help: consider importing this struct
    |
7   | use crate::sys::fs::DirEntry;
    |
    |
help: if you import `DirEntry`, refer to it directly
    |
841 - impl DirEntryExt for fs::DirEntry {
841 + impl DirEntryExt for DirEntry {

error[E0412]: cannot find type `DirEntry` in module `fs`
   --> library/std/src/os/unix/fs.rs:875:21
    |
    |
875 | impl Sealed for fs::DirEntry {}
    |                     ^^^^^^^^ not found in `fs`
help: consider importing this struct
    |
7   | use crate::sys::fs::DirEntry;
    |
    |
help: if you import `DirEntry`, refer to it directly
    |
875 - impl Sealed for fs::DirEntry {}
875 + impl Sealed for DirEntry {}

error[E0412]: cannot find type `DirEntry` in module `fs`
   --> library/std/src/os/unix/fs.rs:878:27
    |
    |
878 | impl DirEntryExt2 for fs::DirEntry {
    |                           ^^^^^^^^ not found in `fs`
help: consider importing this struct
    |
7   | use crate::sys::fs::DirEntry;
    |
    |
help: if you import `DirEntry`, refer to it directly
    |
878 - impl DirEntryExt2 for fs::DirEntry {
878 + impl DirEntryExt2 for DirEntry {

error[E0412]: cannot find type `DirBuilder` in module `fs`
   --> library/std/src/os/unix/fs.rs:923:28
    |
    |
923 | impl DirBuilderExt for fs::DirBuilder {
    |                            ^^^^^^^^^^ not found in `fs`
help: consider importing this struct
    |
7   | use crate::sys::fs::DirBuilder;
    |
    |
help: if you import `DirBuilder`, refer to it directly
    |
923 - impl DirBuilderExt for fs::DirBuilder {
923 + impl DirBuilderExt for DirBuilder {

error[E0412]: cannot find type `DirBuilder` in module `fs`
   --> library/std/src/os/unix/fs.rs:924:47
    |
    |
924 |     fn mode(&mut self, mode: u32) -> &mut fs::DirBuilder {
    |                                               ^^^^^^^^^^ not found in `fs`
help: consider importing this struct
    |
7   | use crate::sys::fs::DirBuilder;
    |
    |
help: if you import `DirBuilder`, refer to it directly
    |
924 -     fn mode(&mut self, mode: u32) -> &mut fs::DirBuilder {
924 +     fn mode(&mut self, mode: u32) -> &mut DirBuilder {

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/raw.rs:161:22
    |
    |
161 | impl AsRawFd for fs::File {
    |                      ^^^^ not found in `fs`
help: consider importing one of these items
    |
5   | use crate::sys::fs::File;
    |
    |
5   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
161 - impl AsRawFd for fs::File {
161 + impl AsRawFd for File {

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/raw.rs:168:24
    |
    |
168 | impl FromRawFd for fs::File {
    |                        ^^^^ not found in `fs`
help: consider importing one of these items
    |
5   | use crate::sys::fs::File;
    |
    |
5   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
168 - impl FromRawFd for fs::File {
168 + impl FromRawFd for File {

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/raw.rs:170:45
    |
    |
170 |     unsafe fn from_raw_fd(fd: RawFd) -> fs::File {
    |                                             ^^^^ not found in `fs`
help: consider importing one of these items
    |
5   | use crate::sys::fs::File;
    |
    |
5   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
170 -     unsafe fn from_raw_fd(fd: RawFd) -> fs::File {
170 +     unsafe fn from_raw_fd(fd: RawFd) -> File {

error[E0433]: failed to resolve: could not find `File` in `fs`
   --> library/std/src/os/fd/raw.rs:171:22
    |
    |
171 |         unsafe { fs::File::from(OwnedFd::from_raw_fd(fd)) }
    |                      ^^^^ not found in `fs`
help: consider importing one of these items
    |
5   | use crate::sys::fs::File;
    |
    |
5   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
171 -         unsafe { fs::File::from(OwnedFd::from_raw_fd(fd)) }
171 +         unsafe { File::from(OwnedFd::from_raw_fd(fd)) }

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/raw.rs:175:24
    |
---

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/owned.rs:259:19
    |
259 | impl AsFd for fs::File {
    |                   ^^^^ not found in `fs`
help: consider importing one of these items
    |
6   | use crate::sys::fs::File;
    |
    |
6   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
259 - impl AsFd for fs::File {
259 + impl AsFd for File {

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/owned.rs:267:15
    |
    |
267 | impl From<fs::File> for OwnedFd {
    |               ^^^^ not found in `fs`
help: consider importing one of these items
    |
6   | use crate::sys::fs::File;
    |
    |
6   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
267 - impl From<fs::File> for OwnedFd {
267 + impl From<File> for OwnedFd {

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/owned.rs:269:23
    |
    |
269 |     fn from(file: fs::File) -> OwnedFd {
    |                       ^^^^ not found in `fs`
help: consider importing one of these items
    |
6   | use crate::sys::fs::File;
    |
    |
6   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
269 -     fn from(file: fs::File) -> OwnedFd {
269 +     fn from(file: File) -> OwnedFd {

error[E0412]: cannot find type `File` in module `fs`
   --> library/std/src/os/fd/owned.rs:275:28
    |
    |
275 | impl From<OwnedFd> for fs::File {
    |                            ^^^^ not found in `fs`
help: consider importing one of these items
    |
6   | use crate::sys::fs::File;
    |
    |
6   | use object::File;
    |
help: if you import `File`, refer to it directly
    |
275 - impl From<OwnedFd> for fs::File {
275 + impl From<OwnedFd> for File {

error[E0412]: cannot find type `Metadata` in module `fs`
    --> library/std/src/path.rs:2621:46
     |
     |
2621 |     pub fn metadata(&self) -> io::Result<fs::Metadata> {
     |                                              ^^^^^^^^ not found in `fs`
     |
help: there is an enum variant `crate::sys::kernel_copy::FdMeta::Metadata` and 1 other; try using the variant's enum
     |
2621 |     pub fn metadata(&self) -> io::Result<crate::sys::kernel_copy::FdMeta> {
     |                                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2621 |     pub fn metadata(&self) -> io::Result<object::SectionKind> {


error[E0425]: cannot find function `metadata` in module `fs`
     |
2622 |         fs::metadata(self)
     |             ^^^^^^^^ not found in `fs`
     |
     |
help: consider importing one of these items
     |
73   | use core::ptr::metadata;
     |
73   | use crate::ptr::metadata;
     |
help: if you import `metadata`, refer to it directly
2622 -         fs::metadata(self)
2622 +         metadata(self)
     |


error[E0412]: cannot find type `Metadata` in module `fs`
    --> library/std/src/path.rs:2640:54
     |
2640 |     pub fn symlink_metadata(&self) -> io::Result<fs::Metadata> {
     |                                                      ^^^^^^^^ not found in `fs`
     |
help: there is an enum variant `crate::sys::kernel_copy::FdMeta::Metadata` and 1 other; try using the variant's enum
     |
2640 |     pub fn symlink_metadata(&self) -> io::Result<crate::sys::kernel_copy::FdMeta> {
     |                                                  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2640 |     pub fn symlink_metadata(&self) -> io::Result<object::SectionKind> {

error[E0425]: cannot find function `symlink_metadata` in module `fs`
    --> library/std/src/path.rs:2641:13
     |
---

error[E0412]: cannot find type `ReadDir` in module `fs`
    --> library/std/src/path.rs:2702:46
     |
2702 |     pub fn read_dir(&self) -> io::Result<fs::ReadDir> {
     |                                              ^^^^^^^ not found in `fs`
help: consider importing this struct
     |
73   | use crate::sys::fs::ReadDir;
     |
     |
help: if you import `ReadDir`, refer to it directly
     |
2702 -     pub fn read_dir(&self) -> io::Result<fs::ReadDir> {
2702 +     pub fn read_dir(&self) -> io::Result<ReadDir> {

error[E0425]: cannot find function `read_dir` in module `fs`
    --> library/std/src/path.rs:2703:13
     |
     |
2703 |         fs::read_dir(self)
     |             ^^^^^^^^ not found in `fs`

error[E0425]: cannot find function `metadata` in module `fs`
     |
     |
2734 |         fs::metadata(self).is_ok()
     |             ^^^^^^^^ not found in `fs`
help: consider importing one of these items
     |
73   | use core::ptr::metadata;
     |
     |
73   | use crate::ptr::metadata;
     |
help: if you import `metadata`, refer to it directly
     |
2734 -         fs::metadata(self).is_ok()
2734 +         metadata(self).is_ok()

error[E0425]: cannot find function `try_exists` in module `fs`
    --> library/std/src/path.rs:2762:13
     |
---
2762 -         fs::try_exists(self)
2762 +         try_exists(self)
     |

error[E0425]: cannot find function `metadata` in module `fs`
     |
     |
2795 |         fs::metadata(self).map(|m| m.is_file()).unwrap_or(false)
     |             ^^^^^^^^ not found in `fs`
help: consider importing one of these items
     |
73   | use core::ptr::metadata;
     |
     |
73   | use crate::ptr::metadata;
     |
help: if you import `metadata`, refer to it directly
     |
2795 -         fs::metadata(self).map(|m| m.is_file()).unwrap_or(false)
2795 +         metadata(self).map(|m| m.is_file()).unwrap_or(false)


error[E0425]: cannot find function `metadata` in module `fs`
     |
     |
2822 |         fs::metadata(self).map(|m| m.is_dir()).unwrap_or(false)
     |             ^^^^^^^^ not found in `fs`
help: consider importing one of these items
     |
73   | use core::ptr::metadata;
     |
     |
73   | use crate::ptr::metadata;
     |
help: if you import `metadata`, refer to it directly
     |
2822 -         fs::metadata(self).map(|m| m.is_dir()).unwrap_or(false)
2822 +         metadata(self).map(|m| m.is_dir()).unwrap_or(false)

error[E0425]: cannot find function `symlink_metadata` in module `fs`
    --> library/std/src/path.rs:2854:13
     |
     |
2854 |         fs::symlink_metadata(self).map(|m| m.is_symlink()).unwrap_or(false)
     |             ^^^^^^^^^^^^^^^^ not found in `fs`
error[E0412]: cannot find type `File` in module `fs`
    --> library/std/src/process.rs:1397:15
     |
     |
1397 | impl From<fs::File> for Stdio {
     |               ^^^^ not found in `fs`
help: consider importing one of these items
     |
107  | use crate::sys::fs::File;
     |
     |
107  | use object::File;
     |
help: if you import `File`, refer to it directly
     |
1397 - impl From<fs::File> for Stdio {
1397 + impl From<File> for Stdio {

error[E0412]: cannot find type `File` in module `fs`
    --> library/std/src/process.rs:1418:23
     |
     |
1418 |     fn from(file: fs::File) -> Stdio {
     |                       ^^^^ not found in `fs`
help: consider importing one of these items
     |
107  | use crate::sys::fs::File;
     |
     |
107  | use object::File;
     |
help: if you import `File`, refer to it directly
     |
1418 -     fn from(file: fs::File) -> Stdio {
1418 +     fn from(file: File) -> Stdio {

error[E0412]: cannot find type `File` in module `crate::fs`
    --> library/std/src/sys/unix/fs.rs:1465:53
     |
     |
1465 | fn open_from(from: &Path) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
     |                                                     ^^^^ not found in `crate::fs`
help: consider importing this struct
     |
1    | use object::File;
     |
     |
help: if you import `File`, refer to it directly
     |
1465 - fn open_from(from: &Path) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
1465 + fn open_from(from: &Path) -> io::Result<(File, crate::fs::Metadata)> {

error[E0412]: cannot find type `Metadata` in module `crate::fs`
    --> library/std/src/sys/unix/fs.rs:1465:70
     |
     |
1465 | fn open_from(from: &Path) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
     |                                                                      ^^^^^^^^ not found in `crate::fs`
     |
help: there is an enum variant `crate::sys::kernel_copy::FdMeta::Metadata` and 1 other; try using the variant's enum
     |
1465 | fn open_from(from: &Path) -> io::Result<(crate::fs::File, crate::sys::kernel_copy::FdMeta)> {
     |                                                           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
1465 | fn open_from(from: &Path) -> io::Result<(crate::fs::File, object::SectionKind)> {

error[E0412]: cannot find type `Metadata` in module `crate::fs`
    --> library/std/src/sys/unix/fs.rs:1491:33
     |
     |
1491 |     reader_metadata: crate::fs::Metadata,
     |                                 ^^^^^^^^ not found in `crate::fs`
     |
help: there is an enum variant `crate::sys::kernel_copy::FdMeta::Metadata` and 1 other; try using the variant's enum
     |
1491 |     reader_metadata: crate::sys::kernel_copy::FdMeta,
1491 |     reader_metadata: object::SectionKind,
     |                      ~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `File` in module `crate::fs`
error[E0412]: cannot find type `File` in module `crate::fs`
    --> library/std/src/sys/unix/fs.rs:1492:29
     |
1492 | ) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
     |                             ^^^^ not found in `crate::fs`
help: consider importing this struct
     |
1    | use object::File;
     |
     |
help: if you import `File`, refer to it directly
     |
1492 - ) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
1492 + ) -> io::Result<(File, crate::fs::Metadata)> {

error[E0412]: cannot find type `Metadata` in module `crate::fs`
    --> library/std/src/sys/unix/fs.rs:1492:46
     |
     |
1492 | ) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
     |                                              ^^^^^^^^ not found in `crate::fs`
     |
help: there is an enum variant `crate::sys::kernel_copy::FdMeta::Metadata` and 1 other; try using the variant's enum
     |
1492 | ) -> io::Result<(crate::fs::File, crate::sys::kernel_copy::FdMeta)> {
     |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
1492 | ) -> io::Result<(crate::fs::File, object::SectionKind)> {

error[E0425]: cannot find function `remove_file` in module `crate::fs`
    --> library/std/src/sys/unix/fs.rs:1863:24
     |
     |
1863 |             crate::fs::remove_file(p)
     |                        ^^^^^^^^^^^ not found in `crate::fs`

error[E0425]: cannot find function `read_link` in module `crate::fs`
   --> library/std/src/sys/unix/os.rs:355:22
    |
355 |     match crate::fs::read_link("/proc/self/exe") {
    |                      ^^^^^^^^^ not found in `crate::fs`
error[E0433]: failed to resolve: could not find `File` in `fs`
  --> library/std/src/sys_common/fs.rs:13:26
   |
   |
13 |     let mut reader = fs::File::open(from)?;
   |                          ^^^^ not found in `fs`
help: consider importing one of these items
   |
3  | use crate::sys::fs::File;
   |
   |
3  | use object::File;
   |
help: if you import `File`, refer to it directly
   |
13 -     let mut reader = fs::File::open(from)?;
13 +     let mut reader = File::open(from)?;

error[E0433]: failed to resolve: could not find `File` in `fs`
  --> library/std/src/sys_common/fs.rs:20:26
   |
   |
20 |     let mut writer = fs::File::create(to)?;
   |                          ^^^^ not found in `fs`
help: consider importing one of these items
   |
3  | use crate::sys::fs::File;
   |
   |
3  | use object::File;
   |
help: if you import `File`, refer to it directly
   |
20 -     let mut writer = fs::File::create(to)?;
20 +     let mut writer = File::create(to)?;

error[E0425]: cannot find function `symlink_metadata` in module `fs`
  --> library/std/src/sys_common/fs.rs:29:24
   |
   |
29 |     let filetype = fs::symlink_metadata(path)?.file_type();
   |                        ^^^^^^^^^^^^^^^^ not found in `fs`
error[E0425]: cannot find function `remove_file` in module `fs`
  --> library/std/src/sys_common/fs.rs:30:36
   |
   |
30 |     if filetype.is_symlink() { fs::remove_file(path) } else { remove_dir_all_recursive(path) }
   |                                    ^^^^^^^^^^^ not found in `fs`
error[E0425]: cannot find function `read_dir` in module `fs`
  --> library/std/src/sys_common/fs.rs:34:22
   |
   |
34 |     for child in fs::read_dir(path)? {
   |                      ^^^^^^^^ not found in `fs`
error[E0425]: cannot find function `remove_file` in module `fs`
  --> library/std/src/sys_common/fs.rs:39:17
   |
   |
39 |             fs::remove_file(&child.path())?;
   |                 ^^^^^^^^^^^ not found in `fs`
error[E0425]: cannot find function `remove_dir` in module `fs`
  --> library/std/src/sys_common/fs.rs:42:9
   |
42 |     fs::remove_dir(path)
42 |     fs::remove_dir(path)
   |         ^^^^^^^^^^ not found in `fs`

error[E0425]: cannot find function `metadata` in module `fs`
  --> library/std/src/sys_common/fs.rs:46:15
   |
46 |     match fs::metadata(path) {
   |               ^^^^^^^^ not found in `fs`
help: consider importing one of these items
   |
3  | use core::ptr::metadata;
   |
   |
3  | use crate::ptr::metadata;
   |
help: if you import `metadata`, refer to it directly
   |
46 -     match fs::metadata(path) {
46 +     match metadata(path) {

error[E0425]: cannot find function `canonicalize` in module `fs`
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:353:20
    |
    |
353 |     let path = fs::canonicalize(path).ok()?;
    |                    ^^^^^^^^^^^^ not found in `fs`
help: consider importing this function
    |
1   | use crate::sys::fs::canonicalize;
    |
    |
help: if you import `canonicalize`, refer to it directly
    |
353 -     let path = fs::canonicalize(path).ok()?;
353 +     let path = canonicalize(path).ok()?;

error[E0425]: cannot find function `canonicalize` in module `fs`
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:413:24
    |
    |
413 |         let path = fs::canonicalize(path).ok()?;
    |                        ^^^^^^^^^^^^ not found in `fs`
help: consider importing this function
    |
1   | use crate::sys::fs::canonicalize;
    |
    |
help: if you import `canonicalize`, refer to it directly
    |
413 -         let path = fs::canonicalize(path).ok()?;
413 +         let path = canonicalize(path).ok()?;

error: unused import: `FromRawFd`
  --> library/std/src/sys/unix/kernel_copy.rs:57:36
   |
   |
57 | use crate::os::unix::io::{AsRawFd, FromRawFd, RawFd};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: module has missing stability attribute
   --> library/std/src/lib.rs:522:1
    |
522 | pub mod fs;
522 | pub mod fs;
    | ^^^^^^^^^^^

error: unused import: `OpenOptionsExt`
    --> library/std/src/sys/unix/fs.rs:1494:31
     |
1494 |     use crate::os::unix::fs::{OpenOptionsExt, PermissionsExt};

error: unused import: `PermissionsExt`
    --> library/std/src/sys/unix/fs.rs:1494:47
     |
     |
1494 |     use crate::os::unix::fs::{OpenOptionsExt, PermissionsExt};

error: unused import: `crate::io::Read`
   --> library/std/src/sys/unix/thread.rs:397:9
    |
---

error: unused import: `AsInner`
  --> library/std/src/os/unix/fs.rs:13:25
   |
13 | use crate::sys_common::{AsInner, AsInnerMut, FromInner};


error: unused import: `AsInnerMut`
   |
   |
13 | use crate::sys_common::{AsInner, AsInnerMut, FromInner};


error: unused import: `super::platform::fs::MetadataExt`
  |
7 | use super::platform::fs::MetadataExt as _;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: unused import: `IntoInner`
  --> library/std/src/os/fd/raw.rs:14:34
   |
14 | use crate::sys_common::{AsInner, IntoInner};

error: unused import: `AsInner`
  --> library/std/src/os/fd/raw.rs:14:25
   |
   |
14 | use crate::sys_common::{AsInner, IntoInner};

error: unused import: `crate::os::unix::fs::FileTypeExt`
  --> library/std/src/sys/unix/kernel_copy.rs:56:5
   |
