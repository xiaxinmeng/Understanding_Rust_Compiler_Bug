
error[E0428]: the name `imp` is defined multiple times
   --> library/std/src/sys/unix/rand.rs:242:1
    |
23  | mod imp {
    | ------- previous definition of the module `imp` here
...
242 | mod imp {
    | ^^^^^^^ `imp` redefined here
    |
    = note: `imp` must be defined only once in the type namespace of this module

error[E0433]: failed to resolve: could not find `platform` in `super`
 --> library/std/src/sys/unix/ext/fs.rs:5:12
  |
5 | use super::platform::fs::MetadataExt as _;
  |            ^^^^^^^^ could not find `platform` in `super`

error[E0433]: failed to resolve: could not find `platform` in `super`
  --> library/std/src/sys/unix/ext/raw.rs:27:16
   |
27 | pub use super::platform::raw::pthread_t;
   |                ^^^^^^^^ could not find `platform` in `super`

error[E0433]: failed to resolve: could not find `platform` in `super`
  --> library/std/src/sys/unix/ext/raw.rs:30:16
   |
30 | pub use super::platform::raw::{blkcnt_t, time_t};
   |                ^^^^^^^^ could not find `platform` in `super`

error[E0433]: failed to resolve: could not find `platform` in `super`
  --> library/std/src/sys/unix/ext/raw.rs:33:16
   |
33 | pub use super::platform::raw::{blksize_t, dev_t, ino_t, mode_t, nlink_t, off_t};
   |                ^^^^^^^^ could not find `platform` in `super`

error[E0432]: unresolved import `crate::os::unix::raw::pthread_t`
 --> library/std/src/sys/unix/ext/thread.rs:6:5
  |
6 | use crate::os::unix::raw::pthread_t;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `pthread_t` in `sys::unix::ext::raw`

error[E0425]: cannot find function `env_lock` in module `sys::os`
  --> library/std/src/sys/unix/process/process_vxworks.rs:71:34
   |
71 |             let _lock = sys::os::env_lock();
   |                                  ^^^^^^^^ not found in `sys::os`

error[E0599]: no method named `st_dev` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:661:14
    |
661 |         self.st_dev()
    |              ^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_ino` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:664:14
    |
664 |         self.st_ino()
    |              ^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_mode` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:667:14
    |
667 |         self.st_mode()
    |              ^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_nlink` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:670:14
    |
670 |         self.st_nlink()
    |              ^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_uid` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:673:14
    |
673 |         self.st_uid()
    |              ^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_gid` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:676:14
    |
676 |         self.st_gid()
    |              ^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_rdev` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:679:14
    |
679 |         self.st_rdev()
    |              ^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_size` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:682:14
    |
682 |         self.st_size()
    |              ^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_atime` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:685:14
    |
685 |         self.st_atime()
    |              ^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_atime_nsec` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:688:14
    |
688 |         self.st_atime_nsec()
    |              ^^^^^^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_mtime` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:691:14
    |
691 |         self.st_mtime()
    |              ^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_mtime_nsec` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:694:14
    |
694 |         self.st_mtime_nsec()
    |              ^^^^^^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_ctime` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:697:14
    |
697 |         self.st_ctime()
    |              ^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_ctime_nsec` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:700:14
    |
700 |         self.st_ctime_nsec()
    |              ^^^^^^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_blksize` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:703:14
    |
703 |         self.st_blksize()
    |              ^^^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_blocks` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:706:14
    |
706 |         self.st_blocks()
    |              ^^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `st_attrib` found for reference `&fs::Metadata` in the current scope
   --> library/std/src/sys/unix/ext/fs.rs:710:14
    |
710 |         self.st_attrib()
    |              ^^^^^^^^^ method not found in `&fs::Metadata`
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::os::vxworks::fs::MetadataExt;`

error[E0599]: no method named `core_dumped` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:259:25
    |
259 |         self.as_inner().core_dumped()
    |                         ^^^^^^^^^^^ method not found in `&process_inner::ExitStatus`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `core_dumped`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:210:1
    |
210 | pub trait ExitStatusExt: Sealed {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `stopped_signal` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:263:25
    |
263 |         self.as_inner().stopped_signal()
    |                         ^^^^^^^^^^^^^^ method not found in `&process_inner::ExitStatus`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `stopped_signal`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:210:1
    |
210 | pub trait ExitStatusExt: Sealed {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `continued` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:267:25
    |
267 |         self.as_inner().continued()
    |                         ^^^^^^^^^ method not found in `&process_inner::ExitStatus`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `continued`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:210:1
    |
210 | pub trait ExitStatusExt: Sealed {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `into_raw` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:271:25
    |
271 |         self.as_inner().into_raw().into()
    |                         ^^^^^^^^ method not found in `&process_inner::ExitStatus`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `into_raw`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:210:1
    |
210 | pub trait ExitStatusExt: Sealed {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 28 previous errors
