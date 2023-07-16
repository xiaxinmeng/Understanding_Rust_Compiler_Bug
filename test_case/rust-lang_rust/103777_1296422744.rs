plain
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `debug` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `debug` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `test_from_utf8` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `test_from_utf8` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
 
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `boxed_pair_to_vec` at RUSTLIB/std/src/macros.rs:LL:CC
---

error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `boxed_pair_to_vec` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `<Allocator as std::alloc::GlobalAlloc>::alloc` at RUSTLIB/std/src/macros.rs:LL:CC
+note: inside `<Allocator as std::alloc::GlobalAlloc>::alloc` at RUSTLIB/std/src/macros.rs:LL:CC
+  --> $DIR/global_allocator.rs:LL:CC
+   |
+LL |             println!("Allocated!")
+   |             ^^^^^^^^^^^^^^^^^^^^^^
+note: inside `_::__rg_alloc` at $DIR/global_allocator.rs:LL:CC
+   |
+LL | #[global_allocator]
+   | ------------------- in this procedural macro expansion
+LL | static ALLOCATOR: Allocator = Allocator;
+LL | static ALLOCATOR: Allocator = Allocator;
+   |                   ^^^^^^^^^
+   = note: inside `std::alloc::alloc` at RUSTLIB/alloc/src/alloc.rs:LL:CC
+   = note: inside `std::alloc::Global::alloc_impl` at RUSTLIB/alloc/src/alloc.rs:LL:CC
+   = note: inside `<std::alloc::Global as std::alloc::Allocator>::allocate` at RUSTLIB/alloc/src/alloc.rs:LL:CC
+note: inside `main` at $DIR/global_allocator.rs:LL:CC
+  --> $DIR/global_allocator.rs:LL:CC
+   |
+LL |     let ptr = Global.allocate(l).unwrap().as_non_null_ptr(); // allocating with Global...
+   |               ^^^^^^^^^^^^^^^^^^
+   = note: this error originates in the macro `println` which comes from the expansion of the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
+error: aborting due to previous error
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:139:22
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:260:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `<Allocator as std::alloc::GlobalAlloc>::alloc` at D:\a\rust\rust\library\std\src\macros.rs:136:9
note: inside `<Allocator as std::alloc::GlobalAlloc>::alloc` at D:\a\rust\rust\library\std\src\macros.rs:136:9
  --> tests/pass\global_allocator.rs:14:13
   |
LL |             println!("Allocated!")
   |             ^^^^^^^^^^^^^^^^^^^^^^
note: inside `_::__rg_alloc` at tests/pass\global_allocator.rs:6:19
   |
LL | #[global_allocator]
   | ------------------- in this procedural macro expansion
LL | static ALLOCATOR: Allocator = Allocator;
LL | static ALLOCATOR: Allocator = Allocator;
   |                   ^^^^^^^^^
   = note: inside `std::alloc::alloc` at D:\a\rust\rust\library\alloc\src\alloc.rs:99:14
   = note: inside `std::alloc::Global::alloc_impl` at D:\a\rust\rust\library\alloc\src\alloc.rs:181:73
   = note: inside `<std::alloc::Global as std::alloc::Allocator>::allocate` at D:\a\rust\rust\library\alloc\src\alloc.rs:241:9
note: inside `main` at tests/pass\global_allocator.rs:32:15
  --> tests/pass\global_allocator.rs:32:15
   |
LL |     let ptr = Global.allocate(l).unwrap().as_non_null_ptr(); // allocating with Global...
   |               ^^^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `println` which comes from the expansion of the attribute macro `global_allocator` (in Nightly builds, run with -Z macro-backtrace for more info)
note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error

---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:139:22
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:260:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:139:22
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:260:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `<InterruptingCow as std::fmt::Display>::fmt` at RUSTLIB/std/src/macros.rs:LL:CC
+note: inside `<InterruptingCow as std::fmt::Display>::fmt` at RUSTLIB/std/src/macros.rs:LL:CC
+  --> $DIR/reentrant-println.rs:LL:CC
+   |
+LL |         println!("Moo");
+   |         ^^^^^^^^^^^^^^^
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `<InterruptingCow as std::fmt::Display>::fmt` at D:\a\rust\rust\library\std\src\macros.rs:136:9
note: inside `<InterruptingCow as std::fmt::Display>::fmt` at D:\a\rust\rust\library\std\src\macros.rs:136:9
  --> tests/pass\reentrant-println.rs:10:9
   |
LL |         println!("Moo");
   |         ^^^^^^^^^^^^^^^
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
  --> tests/pass\reentrant-println.rs:16:5
   |
LL |     println!("\"Knock knock\" \"Who's {} there?\"", InterruptingCow);
   = note: this error originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `start` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:139:22
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:260:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `start` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
-Dropping 0
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `<LoudDrop as std::ops::Drop>::drop` at RUSTLIB/std/src/macros.rs:LL:CC
+   |
+   |
+LL |         eprintln!("Dropping {}", self.0);
+   = note: this error originates in the macro `eprintln` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:421:9
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:171:22
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:955:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1207:21
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:909:9
   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
note: inside `<LoudDrop as std::ops::Drop>::drop` at D:\a\rust\rust\library\std\src\macros.rs:208:9
   |
   |
LL |         eprintln!("Dropping {}", self.0);
   = note: this error originates in the macro `eprintln` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
-$DIR/backtrace-api-v0.rs:LL:CC RUSTLIB/core/src/ops/function.rs:LL:CC (<fn() as std::ops::FnOnce<()>>::call_once - RUSTLIB/std/src/sys_common/backtrace.rs:LL:CC (std::sys_common::backtrace::__rust_begin_short_backtrace)
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::pad` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<str as std::fmt::Display>::fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:421:9
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:171:22
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:955:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::Formatter::<'_>::pad` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1458:20
   = note: inside `<str as std::fmt::Display>::fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:2442:9
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:909:9
   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:208:9
---
-$DIR/backtrace-api-v1.rs:LL:CC RUSTLIB/core/src/ops/function.rs:LL:CC (<fn() as std::ops::FnOnce<()>>::call_once - RUSTLIB/std/src/sys_common/backtrace.rs:LL:CC (std::sys_common::backtrace::__rust_begin_short_backtrace)
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::pad` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<str as std::fmt::Display>::fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:421:9
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:171:22
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:955:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::Formatter::<'_>::pad` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1458:20
   = note: inside `<str as std::fmt::Display>::fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:2442:9
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:909:9
   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:208:9
---
-thread 'childthread' panicked at 'Hello, world!', $DIR/simple.rs:LL:CC
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::sys::PLATFORM::stdio::Stderr> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
---
+   = note: inside `std::sys_common::backtrace::__rust_end_short_backtrace::<[closure@std::rt::begin_panic<&str>::{closure#0}], !>` at RUSTLIB/std/src/sys_common/backtrace.rs:LL:CC
+note: inside closure at RUSTLIB/std/src/panic.rs:LL:CC
+  --> $DIR/simple.rs:LL:CC
+   |
+LL |     let result = thread::spawn(|| panic!("Hello!")).join().unwrap_err();
+   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
---
   = note: inside `std::sys_common::backtrace::__rust_end_short_backtrace::<[closure@std::rt::begin_panic<&str>::{closure#0}], !>` at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:137:18
note: inside closure at D:\a\rust\rust\library\std\src\panic.rs:22:9
  --> tests/pass\concurrency\simple.rs:49:35
   |
LL |     let result = thread::spawn(|| panic!("Hello!")).join().unwrap_err();
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
-Dropping: 5
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
+  --> $DIR/tls_lib_drop_single_thread.rs:LL:CC
+   |
+LL |     eprintln!("Continue main.")
+   = note: this error originates in the macro `eprintln` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:421:9
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:171:22
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:955:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:909:9
   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:208:9
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:208:9
  --> tests/pass\concurrency\tls_lib_drop_single_thread.rs:29:5
   |
LL |     eprintln!("Continue main.")
   = note: this error originates in the macro `eprintln` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `<TestCell as std::ops::Drop>::drop` at RUSTLIB/std/src/macros.rs:LL:CC
+note: inside `<TestCell as std::ops::Drop>::drop` at RUSTLIB/std/src/macros.rs:LL:CC
+  --> $DIR/tls_lib_drop.rs:LL:CC
+   |
+LL |         println!("Dropping: {} (should be before 'Continue main 1').", *self.value.borrow())
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+   = note: inside `std::ptr::drop_in_place::<TestCell> - shim(Some(TestCell))` at RUSTLIB/core/src/ptr/mod.rs:LL:CC
+note: inside `A_CONST::__getit::destroy` at RUSTLIB/std/src/thread/local.rs:LL:CC
+   |
+   |
+LL | / thread_local! {
+LL | |     static A: TestCell = TestCell { value: RefCell::new(0) };
+LL | |     static A_CONST: TestCell = const { TestCell { value: RefCell::new(10) } };
+LL | | }
+   = note: this error originates in the macro `println` which comes from the expansion of the macro `thread_local` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `<TestCell as std::ops::Drop>::drop` at D:\a\rust\rust\library\std\src\macros.rs:136:9
note: inside `<TestCell as std::ops::Drop>::drop` at D:\a\rust\rust\library\std\src\macros.rs:136:9
  --> tests/pass\concurrency\tls_lib_drop.rs:13:9
   |
LL |         println!("Dropping: {} (should be before 'Continue main 1').", *self.value.borrow())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: inside `std::ptr::drop_in_place::<TestCell> - shim(Some(TestCell))` at D:\a\rust\rust\library\core\src\ptr\mod.rs:490:1
note: inside `A_CONST::__getit::destroy` at D:\a\rust\rust\library\std\src\thread\local.rs:232:25
   |
   |
LL | / thread_local! {
LL | |     static A: TestCell = TestCell { value: RefCell::new(0) };
LL | |     static A_CONST: TestCell = const { TestCell { value: RefCell::new(10) } };
LL | | }
   = note: this error originates in the macro `println` which comes from the expansion of the macro `thread_local` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
- at RUSTLIB/std/src/rt.rs:LL:CC
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_char` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::padding` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::pad_integral` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `core::fmt::num::imp::fmt_u64` at RUSTLIB/core/src/fmt/num.rs:LL:CC
+   = note: inside `core::fmt::num::imp::<impl std::fmt::Display for usize>::fmt` at RUSTLIB/core/src/fmt/num.rs:LL:CC
+   = note: inside `core::fmt::run` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_generic` at RUSTLIB/core/src/macros/mod.rs:LL:CC
+   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_with_column` at RUSTLIB/std/src/../../backtrace/src/print.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
+  --> $DIR/backtrace-global-alloc.rs:LL:CC
+   |
+LL |     eprint!("{}", Backtrace::capture());
+   = note: this error originates in the macro `eprint` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:421:9
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:171:22
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:955:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_char` at D:\a\rust\rust\library\core\src\fmt\mod.rs:169:9
   = note: inside `std::fmt::Formatter::<'_>::padding` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1522:13
   = note: inside `std::fmt::Formatter::<'_>::pad_integral` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1418:36
   = note: inside `core::fmt::num::imp::fmt_u64` at D:\a\rust\rust\library\core\src\fmt\num.rs:273:13
   = note: inside `core::fmt::num::imp::<impl std::fmt::Display for usize>::fmt` at D:\a\rust\rust\library\core\src\fmt\num.rs:287:17
   = note: inside `core::fmt::run` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1257:5
   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1663:9
   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1663:9
   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_generic` at D:\a\rust\rust\library\core\src\macros\mod.rs:520:9
   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_with_column` at D:\a\rust\rust\library\std\src\..\..\backtrace\src\print.rs:195:13
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:909:9
   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:170:9
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:170:9
  --> tests/pass\backtrace\backtrace-global-alloc.rs:11:5
   |
LL |     eprint!("{}", Backtrace::capture());
   = note: this error originates in the macro `eprint` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
- at RUSTLIB/std/src/rt.rs:LL:CC
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_char` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::padding` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::pad_integral` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `core::fmt::num::imp::fmt_u64` at RUSTLIB/core/src/fmt/num.rs:LL:CC
+   = note: inside `core::fmt::num::imp::<impl std::fmt::Display for usize>::fmt` at RUSTLIB/core/src/fmt/num.rs:LL:CC
+   = note: inside `core::fmt::run` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_generic` at RUSTLIB/core/src/macros/mod.rs:LL:CC
+   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_with_column` at RUSTLIB/std/src/../../backtrace/src/print.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
+  --> $DIR/backtrace-std.rs:LL:CC
+   |
+LL |     eprint!("{}", func_a());
+   = note: this error originates in the macro `eprint` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:421:9
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:171:22
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:955:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_char` at D:\a\rust\rust\library\core\src\fmt\mod.rs:169:9
   = note: inside `std::fmt::Formatter::<'_>::padding` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1522:13
   = note: inside `std::fmt::Formatter::<'_>::pad_integral` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1418:36
   = note: inside `core::fmt::num::imp::fmt_u64` at D:\a\rust\rust\library\core\src\fmt\num.rs:273:13
   = note: inside `core::fmt::num::imp::<impl std::fmt::Display for usize>::fmt` at D:\a\rust\rust\library\core\src\fmt\num.rs:287:17
   = note: inside `core::fmt::run` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1257:5
   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1663:9
   = note: inside `std::fmt::Formatter::<'_>::write_fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1663:9
   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_generic` at D:\a\rust\rust\library\core\src\macros\mod.rs:520:9
   = note: inside `std::backtrace_rs::print::BacktraceFrameFmt::<'_, '_, '_>::print_raw_with_column` at D:\a\rust\rust\library\std\src\..\..\backtrace\src\print.rs:195:13
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:909:9
   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:170:9
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:170:9
  --> tests/pass\backtrace\backtrace-std.rs:31:5
   |
LL |     eprint!("{}", func_a());
   = note: this error originates in the macro `eprint` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `block_until_complete` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:139:22
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:260:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `block_until_complete` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:139:22
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:260:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `<S as T>::print` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `<S as T>::print` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
-Success!
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::sys::PLATFORM::stdio::Stderr> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside closure at RUSTLIB/core/src/macros/mod.rs:LL:CC
+   = note: inside `std::panicking::default_hook` at RUSTLIB/std/src/panicking.rs:LL:CC
+   = note: inside `<for<'a, 'b> fn(&'a std::panic::PanicInfo<'b>) {std::panicking::default_hook} as std::ops::Fn<(&std::panic::PanicInfo<'_>,)>>::call - shim(for<'a, 'b> fn(&'a std::panic::PanicInfo<'b>) {std::panicking::default_hook})` at RUSTLIB/core/src/ops/function.rs:LL:CC
+   = note: inside `<std::boxed::Box<dyn for<'a, 'b> std::ops::Fn(&'a std::panic::PanicInfo<'b>) + std::marker::Send + std::marker::Sync> as std::ops::Fn<(&std::panic::PanicInfo<'_>,)>>::call` at RUSTLIB/alloc/src/boxed.rs:LL:CC
+  --> $DIR/catch_panic.rs:LL:CC
+   |
+   |
+LL |         prev(panic_info)
+   |         ^^^^^^^^^^^^^^^^
+   = note: inside `<std::boxed::Box<dyn for<'a, 'b> std::ops::Fn(&'a std::panic::PanicInfo<'b>) + std::marker::Send + std::marker::Sync> as std::ops::Fn<(&std::panic::PanicInfo<'_>,)>>::call` at RUSTLIB/alloc/src/boxed.rs:LL:CC
+   = note: inside closure at RUSTLIB/std/src/panicking.rs:LL:CC
+   = note: inside `std::sys_common::backtrace::__rust_end_short_backtrace::<[closure@std::rt::begin_panic<&str>::{closure#0}], !>` at RUSTLIB/std/src/sys_common/backtrace.rs:LL:CC
+note: inside closure at RUSTLIB/std/src/panic.rs:LL:CC
+  --> $DIR/catch_panic.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
---
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1207:21
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside closure at D:\a\rust\rust\library\core\src\macros\mod.rs:558:9
   = note: inside `std::panicking::default_hook` at D:\a\rust\rust\library\std\src\panicking.rs:286:9
   = note: inside `<for<'a, 'b> fn(&'a std::panic::PanicInfo<'b>) {std::panicking::default_hook} as std::ops::Fn<(&std::panic::PanicInfo<'_>,)>>::call - shim(for<'a, 'b> fn(&'a std::panic::PanicInfo<'b>) {std::panicking::default_hook})` at D:\a\rust\rust\library\core\src\ops\function.rs:78:5
   = note: inside `<std::boxed::Box<dyn for<'a, 'b> std::ops::Fn(&'a std::panic::PanicInfo<'b>) + std::marker::Send + std::marker::Sync> as std::ops::Fn<(&std::panic::PanicInfo<'_>,)>>::call` at D:\a\rust\rust\library\alloc\src\boxed.rs:2001:9
  --> tests/pass\panic\catch_panic.rs:47:9
   |
   |
LL |         prev(panic_info)
   |         ^^^^^^^^^^^^^^^^
   = note: inside `<std::boxed::Box<dyn for<'a, 'b> std::ops::Fn(&'a std::panic::PanicInfo<'b>) + std::marker::Send + std::marker::Sync> as std::ops::Fn<(&std::panic::PanicInfo<'_>,)>>::call` at D:\a\rust\rust\library\alloc\src\boxed.rs:2001:9
   = note: inside closure at D:\a\rust\rust\library\std\src\panicking.rs:608:9
   = note: inside `std::sys_common::backtrace::__rust_end_short_backtrace::<[closure@std::rt::begin_panic<&str>::{closure#0}], !>` at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:137:18
note: inside closure at D:\a\rust\rust\library\std\src\panic.rs:22:9
  --> tests/pass\panic\catch_panic.rs:51:27
---
-"hello world" [0, 1, 2, 4]
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stderr as std::io::Write>::write_all` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_char` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::fmt::Formatter<'_> as std::fmt::Write>::write_char` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<str as std::fmt::Debug>::fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<&mut std::string::String as std::fmt::Debug>::fmt` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_eprint` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `disjoint_mutable_subborrows` at RUSTLIB/std/src/macros.rs:LL:CC
+   |
+LL |     eprintln!("{:?} {:?}", a, b);
+   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+note: inside `main` at $DIR/stacked-borrows.rs:LL:CC
+note: inside `main` at $DIR/stacked-borrows.rs:LL:CC
+  --> $DIR/stacked-borrows.rs:LL:CC
+   |
+LL |     disjoint_mutable_subborrows();
+   = note: this error originates in the macro `eprintln` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:421:9
   = note: inside `<std::sys::windows::stdio::Stderr as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\mod.rs:1542:19
   = note: inside `<std::io::stdio::StderrRaw as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:171:22
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:955:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StderrLock<'_>> as std::fmt::Write>::write_char` at D:\a\rust\rust\library\core\src\fmt\mod.rs:169:9
   = note: inside `<std::fmt::Formatter<'_> as std::fmt::Write>::write_char` at D:\a\rust\rust\library\core\src\fmt\mod.rs:2351:9
   = note: inside `<str as std::fmt::Debug>::fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:2417:9
   = note: inside `<&mut std::string::String as std::fmt::Debug>::fmt` at D:\a\rust\rust\library\core\src\fmt\mod.rs:2377:62
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1209:17
   = note: inside `<std::io::StderrLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stderr as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:909:9
   = note: inside `std::io::stdio::print_to::<std::io::Stderr>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
   = note: inside `std::io::_eprint` at D:\a\rust\rust\library\std\src\io\stdio.rs:1086:5
note: inside `disjoint_mutable_subborrows` at D:\a\rust\rust\library\std\src\macros.rs:208:9
   |
LL |     eprintln!("{:?} {:?}", a, b);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `main` at tests/pass\stacked-borrows\stacked-borrows.rs:17:5
note: inside `main` at tests/pass\stacked-borrows\stacked-borrows.rs:17:5
  --> tests/pass\stacked-borrows\stacked-borrows.rs:17:5
   |
LL |     disjoint_mutable_subborrows();
   = note: this error originates in the macro `eprintln` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: NtWriteFile
+  --> RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   |
+LL | /             c::NtWriteFile(
+LL | |                 self.as_handle(),
+LL | |                 ptr::null_mut(),
+LL | |                 None,
+LL | |                 None,
+LL | |             )
+LL | |             )
+   | |_____________^ can't call foreign function: NtWriteFile
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::handle::Handle::synchronous_write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::handle::Handle::write` at RUSTLIB/std/src/sys/PLATFORM/handle.rs:LL:CC
+   = note: inside `<std::sys::PLATFORM::stdio::Stdout as std::io::Write>::write` at RUSTLIB/std/src/sys/PLATFORM/stdio.rs:LL:CC
+   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at RUSTLIB/std/src/io/buffered/bufwriter.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewritershim.rs:LL:CC
+   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at RUSTLIB/std/src/io/buffered/linewriter.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `std::fmt::write` at RUSTLIB/core/src/fmt/mod.rs:LL:CC
+   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/mod.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+   = note: inside `std::io::_print` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at RUSTLIB/std/src/macros.rs:LL:CC
---
full stderr:
error: unsupported operation: can't call foreign function: NtWriteFile
  --> D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   |
LL | /             c::NtWriteFile(
LL | |                 self.as_handle(),
LL | |                 ptr::null_mut(),
LL | |                 None,
LL | |                 None,
LL | |             )
LL | |             )
   | |_____________^ can't call foreign function: NtWriteFile
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: inside `std::sys::windows::handle::Handle::synchronous_write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:290:13
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::handle::Handle::write` at D:\a\rust\rust\library\std\src\sys\windows\handle.rs:196:9
   = note: inside `std::sys::windows::stdio::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:100:23
   = note: inside `<std::sys::windows::stdio::Stdout as std::io::Write>::write` at D:\a\rust\rust\library\std\src\sys\windows\stdio.rs:405:9
   = note: inside `<std::io::stdio::StdoutRaw as std::io::Write>::write` at D:\a\rust\rust\library\std\src\io\stdio.rs:121:22
   = note: inside `std::io::BufWriter::<std::io::stdio::StdoutRaw>::flush_buf` at D:\a\rust\rust\library\std\src\io\buffered\bufwriter.rs:166:21
   = note: inside `<std::io::buffered::linewritershim::LineWriterShim<'_, std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewritershim.rs:269:21
   = note: inside `<std::io::LineWriter<std::io::stdio::StdoutRaw> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\buffered\linewriter.rs:206:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_all` at D:\a\rust\rust\library\std\src\io\stdio.rs:736:9
   = note: inside `<std::io::Write::write_fmt::Adapter<'_, std::io::StdoutLock<'_>> as std::fmt::Write>::write_str` at D:\a\rust\rust\library\std\src\io\mod.rs:1671:23
   = note: inside `std::fmt::write` at D:\a\rust\rust\library\core\src\fmt\mod.rs:1233:9
   = note: inside `<std::io::StdoutLock<'_> as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\mod.rs:1682:15
   = note: inside `<std::io::Stdout as std::io::Write>::write_fmt` at D:\a\rust\rust\library\std\src\io\stdio.rs:690:9
   = note: inside `std::io::stdio::print_to::<std::io::Stdout>` at D:\a\rust\rust\library\std\src\io\stdio.rs:1008:21
   = note: inside `std::io::_print` at D:\a\rust\rust\library\std\src\io\stdio.rs:1075:5
note: inside `main` at D:\a\rust\rust\library\std\src\macros.rs:136:9
