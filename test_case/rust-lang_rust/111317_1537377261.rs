plain
   Compiling addr2line v0.19.0
error[E0433]: failed to resolve: use of undeclared type `Slice`
   --> library/std/src/sys_common/wtf8.rs:813:25
    |
813 |             return Some(Slice::from_u8_slice(suffix));
    |                         ^^^^^ use of undeclared type `Slice`
help: consider importing this struct
    |
21  + use crate::sys::os_str::Slice;
    |
    |

error[E0609]: no field `inner` on type `&'a Wtf8`
   --> library/std/src/sys_common/wtf8.rs:805:25
    |
805 |             return self.inner.starts_with(pattern_bytes);
    |
    = note: available fields are: `bytes`


error[E0609]: no field `inner` on type `&'a Wtf8`
   --> library/std/src/sys_common/wtf8.rs:812:31
    |
812 |             let suffix = self.inner.strip_prefix(prefix_bytes)?;
    |
    = note: available fields are: `bytes`

Some errors have detailed explanations: E0433, E0609.
