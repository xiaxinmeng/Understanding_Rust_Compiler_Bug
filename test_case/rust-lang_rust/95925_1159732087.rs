plain

error[E0432]: unresolved import `crate::sys::fs::File`
  --> library/std/src/sys/unix/process/process_common.rs:13:5
   |
13 | use crate::sys::fs::File;
   |     ^^^^^^^^^^^^^^^^^^^^ no `File` in `sys::unix::fs`
error[E0432]: unresolved import `crate::sys::fs::OpenOptions`
  --> library/std/src/sys/unix/process/process_common.rs:19:5
   |
19 | use crate::sys::fs::OpenOptions;
19 | use crate::sys::fs::OpenOptions;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `OpenOptions` in `sys::unix::fs`

error[E0433]: failed to resolve: could not find `OpenOptions` in `fs_imp`
    |
    |
762 |         OpenOptions(fs_imp::OpenOptions::new())
    |                             ^^^^^^^^^^^ could not find `OpenOptions` in `fs_imp`

error[E0433]: failed to resolve: could not find `DirBuilder` in `fs_imp`
     |
     |
2227 |         DirBuilder { inner: fs_imp::DirBuilder::new(), recursive: false }
     |                                     ^^^^^^^^^^ could not find `DirBuilder` in `fs_imp`

error[E0412]: cannot find type `File` in module `fs_imp`
   |
   |
99 |     inner: fs_imp::File,
   |                    ^^^^ not found in `fs_imp`
help: consider importing this struct
   |
14 | use object::File;
   |
   |
help: if you import `File`, refer to it directly
   |
99 -     inner: fs_imp::File,
99 +     inner: File,


error[E0412]: cannot find type `FileAttr` in module `fs_imp`
    |
    |
110 | pub struct Metadata(fs_imp::FileAttr);
    |                             ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `ReadDir` in module `fs_imp`
    |
    |
128 | pub struct ReadDir(fs_imp::ReadDir);
    |                            ^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `DirEntry` in module `fs_imp`
    |
    |
146 | pub struct DirEntry(fs_imp::DirEntry);
    |                             ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `OpenOptions` in module `fs_imp`
    |
    |
185 | pub struct OpenOptions(fs_imp::OpenOptions);
    |                                ^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FilePermissions` in module `fs_imp`
    |
    |
197 | pub struct Permissions(fs_imp::FilePermissions);
    |                                ^^^^^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FileType` in module `fs_imp`
    |
    |
204 | pub struct FileType(fs_imp::FileType);
    |                             ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `DirBuilder` in module `fs_imp`
    |
    |
213 |     inner: fs_imp::DirBuilder,
    |                    ^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `File` in module `fs_imp`
    |
    |
601 | impl AsInner<fs_imp::File> for File {
    |                      ^^^^ not found in `fs_imp`
help: consider importing this struct
    |
14  | use object::File;
    |
    |
help: if you import `File`, refer to it directly
    |
601 - impl AsInner<fs_imp::File> for File {
601 + impl AsInner<File> for File {


error[E0412]: cannot find type `File` in module `fs_imp`
    |
    |
602 |     fn as_inner(&self) -> &fs_imp::File {
    |                                    ^^^^ not found in `fs_imp`
help: consider importing this struct
    |
14  | use object::File;
    |
    |
help: if you import `File`, refer to it directly
    |
602 -     fn as_inner(&self) -> &fs_imp::File {
602 +     fn as_inner(&self) -> &File {


error[E0412]: cannot find type `File` in module `fs_imp`
    |
    |
606 | impl FromInner<fs_imp::File> for File {
    |                        ^^^^ not found in `fs_imp`
help: consider importing this struct
    |
14  | use object::File;
    |
    |
help: if you import `File`, refer to it directly
    |
606 - impl FromInner<fs_imp::File> for File {
606 + impl FromInner<File> for File {


error[E0412]: cannot find type `File` in module `fs_imp`
    |
    |
607 |     fn from_inner(f: fs_imp::File) -> File {
    |                              ^^^^ not found in `fs_imp`
help: consider importing this struct
    |
14  | use object::File;
    |
    |
help: if you import `File`, refer to it directly
    |
607 -     fn from_inner(f: fs_imp::File) -> File {
607 +     fn from_inner(f: File) -> File {


error[E0412]: cannot find type `File` in module `fs_imp`
    |
    |
611 | impl IntoInner<fs_imp::File> for File {
    |                        ^^^^ not found in `fs_imp`
help: consider importing this struct
    |
14  | use object::File;
    |
    |
help: if you import `File`, refer to it directly
    |
611 - impl IntoInner<fs_imp::File> for File {
611 + impl IntoInner<File> for File {


error[E0412]: cannot find type `File` in module `fs_imp`
    |
    |
612 |     fn into_inner(self) -> fs_imp::File {
    |                                    ^^^^ not found in `fs_imp`
help: consider importing this struct
    |
14  | use object::File;
    |
    |
help: if you import `File`, refer to it directly
    |
612 -     fn into_inner(self) -> fs_imp::File {
612 +     fn into_inner(self) -> File {


error[E0433]: failed to resolve: could not find `File` in `fs_imp`
    |
    |
968 |         fs_imp::File::open(path, &self.0).map(|inner| File { inner })
    |                 ^^^^ not found in `fs_imp`
help: consider importing this struct
    |
14  | use object::File;
    |
    |
help: if you import `File`, refer to it directly
    |
968 -         fs_imp::File::open(path, &self.0).map(|inner| File { inner })
968 +         File::open(path, &self.0).map(|inner| File { inner })


error[E0412]: cannot find type `OpenOptions` in module `fs_imp`
    |
    |
972 | impl AsInner<fs_imp::OpenOptions> for OpenOptions {
    |                      ^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `OpenOptions` in module `fs_imp`
    |
    |
973 |     fn as_inner(&self) -> &fs_imp::OpenOptions {
    |                                    ^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `OpenOptions` in module `fs_imp`
    |
    |
978 | impl AsInnerMut<fs_imp::OpenOptions> for OpenOptions {
    |                         ^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `OpenOptions` in module `fs_imp`
    |
    |
979 |     fn as_inner_mut(&mut self) -> &mut fs_imp::OpenOptions {
    |                                                ^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FileAttr` in module `fs_imp`
     |
     |
1237 | impl AsInner<fs_imp::FileAttr> for Metadata {
     |                      ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FileAttr` in module `fs_imp`
     |
     |
1238 |     fn as_inner(&self) -> &fs_imp::FileAttr {
     |                                    ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FileAttr` in module `fs_imp`
     |
     |
1243 | impl FromInner<fs_imp::FileAttr> for Metadata {
     |                        ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FileAttr` in module `fs_imp`
     |
     |
1244 |     fn from_inner(attr: fs_imp::FileAttr) -> Metadata {
     |                                 ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FileType` in module `fs_imp`
     |
     |
1404 | impl AsInner<fs_imp::FileType> for FileType {
     |                      ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FileType` in module `fs_imp`
     |
     |
1405 |     fn as_inner(&self) -> &fs_imp::FileType {
     |                                    ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FilePermissions` in module `fs_imp`
     |
     |
1410 | impl FromInner<fs_imp::FilePermissions> for Permissions {
     |                        ^^^^^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FilePermissions` in module `fs_imp`
     |
     |
1411 |     fn from_inner(f: fs_imp::FilePermissions) -> Permissions {
     |                              ^^^^^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FilePermissions` in module `fs_imp`
     |
     |
1416 | impl AsInner<fs_imp::FilePermissions> for Permissions {
     |                      ^^^^^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `FilePermissions` in module `fs_imp`
     |
     |
1417 |     fn as_inner(&self) -> &fs_imp::FilePermissions {
     |                                    ^^^^^^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `DirEntry` in module `fs_imp`
     |
     |
1570 | impl AsInner<fs_imp::DirEntry> for DirEntry {
     |                      ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `DirEntry` in module `fs_imp`
     |
     |
1571 |     fn as_inner(&self) -> &fs_imp::DirEntry {
     |                                    ^^^^^^^^ not found in `fs_imp`

error[E0425]: cannot find function `unlink` in module `fs_imp`
     |
     |
1611 |     fs_imp::unlink(path.as_ref())
     |             ^^^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use libc::unlink;
     |
     |
help: if you import `unlink`, refer to it directly
     |
1611 -     fs_imp::unlink(path.as_ref())
1611 +     unlink(path.as_ref())


error[E0425]: cannot find function `stat` in module `fs_imp`
     |
     |
1649 |     fs_imp::stat(path.as_ref()).map(Metadata)
     |             ^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use libc::stat;
     |
     |
help: if you import `stat`, refer to it directly
     |
1649 -     fs_imp::stat(path.as_ref()).map(Metadata)
1649 +     stat(path.as_ref()).map(Metadata)


error[E0425]: cannot find function `lstat` in module `fs_imp`
     |
     |
1683 |     fs_imp::lstat(path.as_ref()).map(Metadata)
     |             ^^^^^ not found in `fs_imp`
help: consider importing this function
     |
     |
14   | use libc::lstat;
     |
help: if you import `lstat`, refer to it directly
     |
1683 -     fs_imp::lstat(path.as_ref()).map(Metadata)
1683 +     lstat(path.as_ref()).map(Metadata)


error[E0425]: cannot find function `rename` in module `fs_imp`
     |
     |
1726 |     fs_imp::rename(from.as_ref(), to.as_ref())
     |             ^^^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use libc::rename;
     |
     |
help: if you import `rename`, refer to it directly
     |
1726 -     fs_imp::rename(from.as_ref(), to.as_ref())
1726 +     rename(from.as_ref(), to.as_ref())


error[E0425]: cannot find function `copy` in module `fs_imp`
     |
     |
1784 |     fs_imp::copy(from.as_ref(), to.as_ref())
     |             ^^^^ not found in `fs_imp`
help: consider importing one of these items
     |
14   | use core::ptr::copy;
     |
     |
14   | use crate::io::copy;
     |
14   | use crate::ptr::copy;
     |
14   | use crate::sys_common::fs::copy;
     |
help: if you import `copy`, refer to it directly
     |
1784 -     fs_imp::copy(from.as_ref(), to.as_ref())
1784 +     copy(from.as_ref(), to.as_ref())


error[E0425]: cannot find function `link` in module `fs_imp`
     |
     |
1828 |     fs_imp::link(original.as_ref(), link.as_ref())
     |             ^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use libc::link;
     |
     |
help: if you import `link`, refer to it directly
     |
1828 -     fs_imp::link(original.as_ref(), link.as_ref())
1828 +     link(original.as_ref(), link.as_ref())


error[E0425]: cannot find function `symlink` in module `fs_imp`
     |
     |
1860 |     fs_imp::symlink(original.as_ref(), link.as_ref())
     |             ^^^^^^^ not found in `fs_imp`
help: consider importing one of these items
     |
14   | use crate::os::unix::fs::symlink;
     |
     |
14   | use libc::symlink;
     |
help: if you import `symlink`, refer to it directly
     |
1860 -     fs_imp::symlink(original.as_ref(), link.as_ref())
1860 +     symlink(original.as_ref(), link.as_ref())


error[E0425]: cannot find function `readlink` in module `fs_imp`
     |
     |
1894 |     fs_imp::readlink(path.as_ref())
     |             ^^^^^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use libc::readlink;
     |
     |
help: if you import `readlink`, refer to it directly
     |
1894 -     fs_imp::readlink(path.as_ref())
1894 +     readlink(path.as_ref())


error[E0425]: cannot find function `canonicalize` in module `fs_imp`
     |
     |
1937 |     fs_imp::canonicalize(path.as_ref())
     |             ^^^^^^^^^^^^ not found in `fs_imp`

error[E0425]: cannot find function `rmdir` in module `fs_imp`
     |
     |
2058 |     fs_imp::rmdir(path.as_ref())
     |             ^^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use libc::rmdir;
     |
     |
help: if you import `rmdir`, refer to it directly
     |
2058 -     fs_imp::rmdir(path.as_ref())
2058 +     rmdir(path.as_ref())


error[E0425]: cannot find function `remove_dir_all` in module `fs_imp`
     |
     |
2100 |     fs_imp::remove_dir_all(path.as_ref())
     |             ^^^^^^^^^^^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use crate::sys_common::fs::remove_dir_all;
     |
     |
help: if you import `remove_dir_all`, refer to it directly
     |
2100 -     fs_imp::remove_dir_all(path.as_ref())
2100 +     remove_dir_all(path.as_ref())


error[E0425]: cannot find function `readdir` in module `fs_imp`
     |
     |
2175 |     fs_imp::readdir(path.as_ref()).map(ReadDir)
     |             ^^^^^^^ not found in `fs_imp`
help: consider importing this function
     |
14   | use libc::readdir;
     |
     |
help: if you import `readdir`, refer to it directly
     |
2175 -     fs_imp::readdir(path.as_ref()).map(ReadDir)
2175 +     readdir(path.as_ref()).map(ReadDir)


error[E0425]: cannot find function `set_perm` in module `fs_imp`
     |
     |
2210 |     fs_imp::set_perm(path.as_ref(), perm.0)
     |             ^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `DirBuilder` in module `fs_imp`
     |
     |
2305 | impl AsInnerMut<fs_imp::DirBuilder> for DirBuilder {
     |                         ^^^^^^^^^^ not found in `fs_imp`

error[E0412]: cannot find type `DirBuilder` in module `fs_imp`
     |
     |
2306 |     fn as_inner_mut(&mut self) -> &mut fs_imp::DirBuilder {
     |                                                ^^^^^^^^^^ not found in `fs_imp`

error[E0425]: cannot find function `try_exists` in module `fs_imp`
