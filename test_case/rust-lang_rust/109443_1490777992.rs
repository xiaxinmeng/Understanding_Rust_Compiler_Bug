plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Rustbook (x86_64-unknown-linux-gnu) - edition-guide
Rustbook (x86_64-unknown-linux-gnu) - style-guide
Building tool linkchecker (stage0)
    Finished release [optimized] target(s) in 0.13s
std/primitive.slice.html:4: broken link - `std/crate::slice`
std/primitive.slice.html:34: broken link - `std/core::hash::Hash`
std/primitive.slice.html:53: broken link - `std/slice::iter`
std/primitive.slice.html:53: broken link - `std/slice::iter_mut`
std/primitive.slice.html:55: broken link - `std/slice::split`
std/primitive.slice.html:55: broken link - `std/slice::splitn`
std/primitive.slice.html:56: broken link - `std/slice::chunks`
std/primitive.slice.html:56: broken link - `std/slice::windows`
std/primitive.slice.html:34: broken intra-doc link - [<code>Eq</code>], <a href="core::hash::Hash"><code>Hash</code></a> and [<code>Ord</code>]
std/primitive.tuple.html:37: broken link - `std/fmt::Debug`
std/primitive.tuple.html:39: broken link - `std/hash::Hash`
std/primitive.tuple.html:49: broken link - `std/marker::Unpin`
std/primitive.tuple.html:50: broken link - `std/panic::UnwindSafe`
std/primitive.tuple.html:51: broken link - `std/panic::RefUnwindSafe`
std/primitive.tuple.html:22: broken intra-doc link - [<code>PartialOrd</code>] and [<code>Ord</code>]
std/primitive.tuple.html:33: broken intra-doc link - [<code>PartialEq</code>]
std/primitive.tuple.html:34: broken intra-doc link - [<code>Eq</code>]
std/primitive.tuple.html:35: broken intra-doc link - [<code>PartialOrd</code>]
std/primitive.tuple.html:36: broken intra-doc link - [<code>Ord</code>]
std/primitive.tuple.html:38: broken intra-doc link - [<code>Default</code>]
std/primitive.tuple.html:45: broken intra-doc link - [<code>Clone</code>]
std/primitive.tuple.html:46: broken intra-doc link - [<code>Copy</code>]
std/primitive.tuple.html:47: broken intra-doc link - [<code>Send</code>]
std/primitive.tuple.html:48: broken intra-doc link - [<code>Sync</code>]
std/primitive.reference.html:7: broken link - `std/prim@bool`
std/primitive.reference.html:9: broken link - `std/prim@bool`
std/primitive.reference.html:49: broken link - `std/ops::Deref`
std/primitive.reference.html:50: broken link - `std/borrow::Borrow`
std/primitive.reference.html:57: broken link - `std/ops::DerefMut`
std/primitive.reference.html:58: broken link - `std/borrow::BorrowMut`
std/primitive.reference.html:63: directory link to `std/fmt` (directory links should use index.html instead)
std/primitive.reference.html:70: broken link - `std/hash::Hash`
std/primitive.reference.html:71: broken link - `std/net::ToSocketAddrs`
std/primitive.reference.html:83: broken link - `std/iter::FusedIterator`
std/primitive.reference.html:84: broken link - `std/iter::TrustedLen`
std/primitive.reference.html:85: broken link - `std/io::Write`
std/primitive.reference.html:86: broken link - `std/io::Read`
std/primitive.reference.html:87: broken link - `std/io::Seek`
std/primitive.reference.html:88: broken link - `std/io::BufRead`
std/primitive.reference.html:26: broken intra-doc link - [<code>ptr::eq</code>]
std/primitive.reference.html:27: broken intra-doc link - [<code>PartialEq</code>]
std/primitive.reference.html:47: broken intra-doc link - [<code>Copy</code>]
std/primitive.reference.html:48: broken intra-doc link - [<code>Clone</code>]
std/primitive.reference.html:51: broken intra-doc link - [<code>fmt::Pointer</code>]
std/primitive.reference.html:63: broken intra-doc link - [<code>fmt::Pointer</code>] (which is implemented regardless of the type of its referent) and [<code>fmt::Write</code>]
std/primitive.reference.html:64: broken intra-doc link - [<code>PartialOrd</code>]
std/primitive.reference.html:65: broken intra-doc link - [<code>Ord</code>]
std/primitive.reference.html:66: broken intra-doc link - [<code>PartialEq</code>]
std/primitive.reference.html:67: broken intra-doc link - [<code>Eq</code>]
std/primitive.reference.html:68: broken intra-doc link - [<code>AsRef</code>]
std/primitive.reference.html:69: broken intra-doc link - [<code>Fn</code>] (in addition, <code>&amp;T</code> references get [<code>FnMut</code>] and [<code>FnOnce</code>]
std/primitive.reference.html:72: broken intra-doc link - [<code>Send</code>]
std/primitive.reference.html:77: broken intra-doc link - [<code>AsMut</code>]
std/primitive.reference.html:78: broken intra-doc link - [<code>FnMut</code>] (in addition, <code>&amp;mut T</code> references get [<code>FnOnce</code>]
std/primitive.reference.html:79: broken intra-doc link - [<code>fmt::Write</code>]
std/primitive.reference.html:80: broken intra-doc link - [<code>Iterator</code>]
std/primitive.reference.html:81: broken intra-doc link - [<code>DoubleEndedIterator</code>]
std/primitive.reference.html:82: broken intra-doc link - [<code>ExactSizeIterator</code>]
std/primitive.f32.html:59: broken link - `std/crate::f32::consts`
std/primitive.char.html:43: broken link - `std/string::String`
std/primitive.bool.html:5: broken link - `std/ops::BitAnd`
std/primitive.bool.html:5: broken link - `std/ops::BitOr`
std/primitive.bool.html:5: broken link - `std/ops::Not`
std/primitive.bool.html:7: broken intra-doc link - [<code>assert!</code>]
std/primitive.bool.html:30: broken intra-doc link - [<code>Copy</code>]
std/primitive.pointer.html:2: directory link to `std/ptr` (directory links should use index.html instead)
std/primitive.pointer.html:4: broken link - `std/ptr::null`
std/primitive.pointer.html:7: broken link - `std/ptr::write`
std/primitive.pointer.html:9: broken link - `std/ptr::null`
std/primitive.pointer.html:9: broken link - `std/ptr::null_mut`
std/primitive.pointer.html:10: broken link - `std/pointer::is_null`
std/primitive.pointer.html:11: broken link - `std/pointer::offset`
std/primitive.pointer.html:28: broken link - `std/Box::into_raw`
std/primitive.pointer.html:39: broken link - `std/mem::drop`
std/primitive.pointer.html:43: broken intra-doc link - [<code>ptr::addr_of!</code>] (for <code>*const T</code>) and [<code>ptr::addr_of_mut!</code>]
std/primitive.f64.html:2: broken link - `std/prim@f32`
std/primitive.f64.html:3: broken link - `std/prim@f32`
std/primitive.f64.html:6: broken link - `std/crate::f64::consts`
std/primitive.array.html:23: broken link - `std/fmt::Debug`
std/primitive.array.html:26: broken link - `std/hash::Hash`
std/primitive.array.html:28: broken link - `std/borrow::Borrow`
std/primitive.array.html:28: broken link - `std/borrow::BorrowMut`
std/primitive.array.html:33: broken link - `std/prim@slice`
std/primitive.array.html:41: broken link - `std/crate::convert::TryFrom`
std/primitive.array.html:81: broken link - `std/slice::iter`
std/primitive.array.html:11: broken intra-doc link - [<code>Copy</code>]
std/primitive.array.html:21: broken intra-doc link - [<code>Copy</code>]
std/primitive.array.html:22: broken intra-doc link - [<code>Clone</code>]
std/primitive.array.html:24: broken intra-doc link - [<code>IntoIterator</code>]
std/primitive.array.html:25: broken intra-doc link - [<code>PartialEq</code>], [<code>PartialOrd</code>], [<code>Eq</code>], [<code>Ord</code>]
std/primitive.array.html:27: broken intra-doc link - [<code>AsRef</code>], [<code>AsMut</code>]
std/primitive.array.html:30: broken intra-doc link - [<code>Default</code>]
std/primitive.array.html:48: broken intra-doc link - [<code>mem::replace</code>]
std/primitive.array.html:80: broken intra-doc link - [<code>IntoIterator</code>]
std/primitive.array.html:83: broken intra-doc link - [<code>IntoIterator</code>]
std/primitive.array.html:133: broken intra-doc link - [<code>IntoIterator::into_iter</code>]
std/primitive.fn.html:2: broken link - `std/ops::Fn`
std/primitive.fn.html:2: broken link - `std/ops::FnMut`
std/primitive.fn.html:2: broken link - `std/ops::FnOnce`
std/primitive.fn.html:6: broken link - `std/core::option`
std/primitive.fn.html:106: broken link - `std/hash::Hash`
std/primitive.fn.html:107: broken link - `std/fmt::Pointer`
std/primitive.fn.html:119: broken link - `std/panic::UnwindSafe`
std/primitive.fn.html:120: broken link - `std/panic::RefUnwindSafe`
std/primitive.fn.html:122: broken link - `std/ops::Fn`
std/primitive.fn.html:122: broken link - `std/ops::FnMut`
std/primitive.fn.html:122: broken link - `std/ops::FnOnce`
std/primitive.fn.html:102: broken intra-doc link - [<code>PartialEq</code>]
std/primitive.fn.html:103: broken intra-doc link - [<code>Eq</code>]
std/primitive.fn.html:104: broken intra-doc link - [<code>PartialOrd</code>]
std/primitive.fn.html:105: broken intra-doc link - [<code>Ord</code>]
std/primitive.fn.html:108: broken intra-doc link - [<code>Debug</code>]
std/primitive.fn.html:114: broken intra-doc link - [<code>Clone</code>]
std/primitive.fn.html:115: broken intra-doc link - [<code>Copy</code>]
std/primitive.fn.html:116: broken intra-doc link - [<code>Send</code>]
std/primitive.fn.html:117: broken intra-doc link - [<code>Sync</code>]
std/primitive.fn.html:118: broken intra-doc link - [<code>Unpin</code>]
std/primitive.str.html:2: broken link - `std/crate::str`
std/primitive.str.html:19: broken link - `std/str::as_ptr`
std/primitive.str.html:19: broken link - `std/str::len`
std/primitive.never.html:3: broken link - `std/process::exit`
std/primitive.never.html:22: broken link - `std/prim@u32`
std/primitive.never.html:23: broken link - `std/prim@u32`
std/primitive.never.html:26: broken link - `std/str::FromStr`
std/primitive.never.html:33: broken link - `std/string::String`
std/primitive.never.html:37: broken link - `std/str::FromStr::from_str`
std/primitive.never.html:112: broken link - `std/fmt::Debug`
std/primitive.never.html:134: broken link - `std/fs::File`
std/primitive.never.html:134: broken link - `std/Default::default`
std/primitive.never.html:33: broken intra-doc link - [<code>Err</code>]
std/primitive.never.html:36: broken intra-doc link - [<code>Err</code>]
std/primitive.never.html:38: broken intra-doc link - [<code>Result&lt;String, !&gt;</code>]
std/primitive.never.html:43: broken intra-doc link - [<code>Err</code>]
std/primitive.never.html:44: broken intra-doc link - [<code>Result&lt;T, !&gt;</code>]
std/primitive.never.html:45: broken intra-doc link - [<code>Ok</code>]
std/primitive.never.html:48: broken intra-doc link - [<code>Result&lt;T, !&gt;</code>]
std/primitive.never.html:49: broken intra-doc link - [<code>Result&lt;T, !&gt;</code>]
std/primitive.never.html:50: broken intra-doc link - [<code>Result&lt;!, E&gt;</code>]
std/primitive.never.html:72: broken intra-doc link - [<code>Result&lt;!, E&gt;</code>]
std/primitive.never.html:122: broken intra-doc link - [<code>fmt::Result</code>]
std/primitive.never.html:125: broken intra-doc link - [<code>fmt::Result</code>]
std/primitive.never.html:128: broken intra-doc link - [<code>Default</code>]
core/primitive.slice.html:4: broken link - `core/crate::slice`
core/primitive.slice.html:34: broken link - `core/core::hash::Hash`
core/primitive.slice.html:53: broken link - `core/slice::iter`
core/primitive.slice.html:53: broken link - `core/slice::iter_mut`
core/primitive.slice.html:55: broken link - `core/slice::split`
core/primitive.slice.html:55: broken link - `core/slice::splitn`
core/primitive.slice.html:56: broken link - `core/slice::chunks`
core/primitive.slice.html:56: broken link - `core/slice::windows`
core/primitive.slice.html:34: broken intra-doc link - [<code>Eq</code>], <a href="core::hash::Hash"><code>Hash</code></a> and [<code>Ord</code>]
core/primitive.tuple.html:37: broken link - `core/fmt::Debug`
core/primitive.tuple.html:39: broken link - `core/hash::Hash`
core/primitive.tuple.html:49: broken link - `core/marker::Unpin`
core/primitive.tuple.html:50: broken link - `core/panic::UnwindSafe`
core/primitive.tuple.html:51: broken link - `core/panic::RefUnwindSafe`
core/primitive.tuple.html:22: broken intra-doc link - [<code>PartialOrd</code>] and [<code>Ord</code>]
core/primitive.tuple.html:33: broken intra-doc link - [<code>PartialEq</code>]
core/primitive.tuple.html:34: broken intra-doc link - [<code>Eq</code>]
core/primitive.tuple.html:35: broken intra-doc link - [<code>PartialOrd</code>]
core/primitive.tuple.html:36: broken intra-doc link - [<code>Ord</code>]
core/primitive.tuple.html:38: broken intra-doc link - [<code>Default</code>]
core/primitive.tuple.html:45: broken intra-doc link - [<code>Clone</code>]
core/primitive.tuple.html:46: broken intra-doc link - [<code>Copy</code>]
core/primitive.tuple.html:47: broken intra-doc link - [<code>Send</code>]
core/primitive.tuple.html:48: broken intra-doc link - [<code>Sync</code>]
core/primitive.reference.html:7: broken link - `core/prim@bool`
core/primitive.reference.html:9: broken link - `core/prim@bool`
core/primitive.reference.html:49: broken link - `core/ops::Deref`
core/primitive.reference.html:50: broken link - `core/borrow::Borrow`
core/primitive.reference.html:57: broken link - `core/ops::DerefMut`
core/primitive.reference.html:58: broken link - `core/borrow::BorrowMut`
core/primitive.reference.html:63: directory link to `core/fmt` (directory links should use index.html instead)
core/primitive.reference.html:70: broken link - `core/hash::Hash`
core/primitive.reference.html:83: broken link - `core/iter::FusedIterator`
core/primitive.reference.html:84: broken link - `core/iter::TrustedLen`
core/primitive.reference.html:26: broken intra-doc link - [<code>ptr::eq</code>]
core/primitive.reference.html:27: broken intra-doc link - [<code>PartialEq</code>]
core/primitive.reference.html:47: broken intra-doc link - [<code>Copy</code>]
core/primitive.reference.html:48: broken intra-doc link - [<code>Clone</code>]
core/primitive.reference.html:51: broken intra-doc link - [<code>fmt::Pointer</code>]
core/primitive.reference.html:63: broken intra-doc link - [<code>fmt::Pointer</code>] (which is implemented regardless of the type of its referent) and [<code>fmt::Write</code>]
core/primitive.reference.html:64: broken intra-doc link - [<code>PartialOrd</code>]
core/primitive.reference.html:65: broken intra-doc link - [<code>Ord</code>]
core/primitive.reference.html:66: broken intra-doc link - [<code>PartialEq</code>]
core/primitive.reference.html:67: broken intra-doc link - [<code>Eq</code>]
core/primitive.reference.html:68: broken intra-doc link - [<code>AsRef</code>]
core/primitive.reference.html:69: broken intra-doc link - [<code>Fn</code>] (in addition, <code>&amp;T</code> references get [<code>FnMut</code>] and [<code>FnOnce</code>]
core/primitive.reference.html:72: broken intra-doc link - [<code>Send</code>]
core/primitive.reference.html:77: broken intra-doc link - [<code>AsMut</code>]
core/primitive.reference.html:78: broken intra-doc link - [<code>FnMut</code>] (in addition, <code>&amp;mut T</code> references get [<code>FnOnce</code>]
core/primitive.reference.html:79: broken intra-doc link - [<code>fmt::Write</code>]
core/primitive.reference.html:80: broken intra-doc link - [<code>Iterator</code>]
core/primitive.reference.html:81: broken intra-doc link - [<code>DoubleEndedIterator</code>]
core/primitive.reference.html:82: broken intra-doc link - [<code>ExactSizeIterator</code>]
core/primitive.f32.html:59: broken link - `core/crate::f32::consts`
core/primitive.bool.html:5: broken link - `core/ops::BitAnd`
core/primitive.bool.html:5: broken link - `core/ops::BitOr`
core/primitive.bool.html:5: broken link - `core/ops::Not`
core/primitive.bool.html:7: broken intra-doc link - [<code>assert!</code>]
core/primitive.bool.html:30: broken intra-doc link - [<code>Copy</code>]
core/primitive.pointer.html:2: directory link to `core/ptr` (directory links should use index.html instead)
core/primitive.pointer.html:4: broken link - `core/ptr::null`
core/primitive.pointer.html:7: broken link - `core/ptr::write`
core/primitive.pointer.html:9: broken link - `core/ptr::null`
core/primitive.pointer.html:9: broken link - `core/ptr::null_mut`
core/primitive.pointer.html:10: broken link - `core/pointer::is_null`
core/primitive.pointer.html:11: broken link - `core/pointer::offset`
core/primitive.pointer.html:39: broken link - `core/mem::drop`
core/primitive.pointer.html:43: broken intra-doc link - [<code>ptr::addr_of!</code>] (for <code>*const T</code>) and [<code>ptr::addr_of_mut!</code>]
core/primitive.f64.html:2: broken link - `core/prim@f32`
core/primitive.f64.html:3: broken link - `core/prim@f32`
core/primitive.f64.html:6: broken link - `core/crate::f64::consts`
core/primitive.array.html:23: broken link - `core/fmt::Debug`
core/primitive.array.html:26: broken link - `core/hash::Hash`
core/primitive.array.html:28: broken link - `core/borrow::Borrow`
core/primitive.array.html:28: broken link - `core/borrow::BorrowMut`
core/primitive.array.html:33: broken link - `core/prim@slice`
core/primitive.array.html:41: broken link - `core/crate::convert::TryFrom`
core/primitive.array.html:81: broken link - `core/slice::iter`
core/primitive.array.html:11: broken intra-doc link - [<code>Copy</code>]
core/primitive.array.html:21: broken intra-doc link - [<code>Copy</code>]
core/primitive.array.html:22: broken intra-doc link - [<code>Clone</code>]
core/primitive.array.html:24: broken intra-doc link - [<code>IntoIterator</code>]
core/primitive.array.html:25: broken intra-doc link - [<code>PartialEq</code>], [<code>PartialOrd</code>], [<code>Eq</code>], [<code>Ord</code>]
core/primitive.array.html:27: broken intra-doc link - [<code>AsRef</code>], [<code>AsMut</code>]
core/primitive.array.html:30: broken intra-doc link - [<code>Default</code>]
core/primitive.array.html:48: broken intra-doc link - [<code>mem::replace</code>]
core/primitive.array.html:80: broken intra-doc link - [<code>IntoIterator</code>]
core/primitive.array.html:83: broken intra-doc link - [<code>IntoIterator</code>]
core/primitive.array.html:133: broken intra-doc link - [<code>IntoIterator::into_iter</code>]
core/primitive.fn.html:2: broken link - `core/ops::Fn`
core/primitive.fn.html:2: broken link - `core/ops::FnMut`
core/primitive.fn.html:2: broken link - `core/ops::FnOnce`
core/primitive.fn.html:6: broken link - `core/core::option`
core/primitive.fn.html:106: broken link - `core/hash::Hash`
core/primitive.fn.html:107: broken link - `core/fmt::Pointer`
core/primitive.fn.html:119: broken link - `core/panic::UnwindSafe`
core/primitive.fn.html:120: broken link - `core/panic::RefUnwindSafe`
core/primitive.fn.html:122: broken link - `core/ops::Fn`
core/primitive.fn.html:122: broken link - `core/ops::FnMut`
core/primitive.fn.html:122: broken link - `core/ops::FnOnce`
core/primitive.fn.html:102: broken intra-doc link - [<code>PartialEq</code>]
core/primitive.fn.html:103: broken intra-doc link - [<code>Eq</code>]
core/primitive.fn.html:104: broken intra-doc link - [<code>PartialOrd</code>]
core/primitive.fn.html:105: broken intra-doc link - [<code>Ord</code>]
core/primitive.fn.html:108: broken intra-doc link - [<code>Debug</code>]
core/primitive.fn.html:114: broken intra-doc link - [<code>Clone</code>]
core/primitive.fn.html:115: broken intra-doc link - [<code>Copy</code>]
core/primitive.fn.html:116: broken intra-doc link - [<code>Send</code>]
core/primitive.fn.html:117: broken intra-doc link - [<code>Sync</code>]
core/primitive.fn.html:118: broken intra-doc link - [<code>Unpin</code>]
core/primitive.str.html:2: broken link - `core/crate::str`
core/primitive.str.html:19: broken link - `core/str::as_ptr`
core/primitive.str.html:19: broken link - `core/str::len`
core/primitive.never.html:22: broken link - `core/prim@u32`
core/primitive.never.html:23: broken link - `core/prim@u32`
core/primitive.never.html:26: broken link - `core/str::FromStr`
core/primitive.never.html:37: broken link - `core/str::FromStr::from_str`
core/primitive.never.html:112: broken link - `core/fmt::Debug`
core/primitive.never.html:134: broken link - `core/Default::default`
core/primitive.never.html:33: broken intra-doc link - [<code>Err</code>]
core/primitive.never.html:36: broken intra-doc link - [<code>Err</code>]
core/primitive.never.html:38: broken intra-doc link - [<code>Result&lt;String, !&gt;</code>]
core/primitive.never.html:43: broken intra-doc link - [<code>Err</code>]
core/primitive.never.html:44: broken intra-doc link - [<code>Result&lt;T, !&gt;</code>]
core/primitive.never.html:45: broken intra-doc link - [<code>Ok</code>]
core/primitive.never.html:48: broken intra-doc link - [<code>Result&lt;T, !&gt;</code>]
core/primitive.never.html:49: broken intra-doc link - [<code>Result&lt;T, !&gt;</code>]
core/primitive.never.html:50: broken intra-doc link - [<code>Result&lt;!, E&gt;</code>]
core/primitive.never.html:72: broken intra-doc link - [<code>Result&lt;!, E&gt;</code>]
core/primitive.never.html:122: broken intra-doc link - [<code>fmt::Result</code>]
core/primitive.never.html:125: broken intra-doc link - [<code>fmt::Result</code>]
core/primitive.never.html:128: broken intra-doc link - [<code>Default</code>]
number of HTML files scanned: 33594
number of HTML redirects found: 10272
number of links checked: 2954543
number of links ignored due to external: 97095
