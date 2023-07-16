
src/libcore/macros.rs:41:            static _FILE_LINE: (&'static str, uint) = (file!(), line!());
src/libcore/macros.rs:42:            ::core::failure::begin_unwind(fmt, &_FILE_LINE)
src/libstd/macros.rs:46:        static _FILE_LINE: (&'static str, uint) = (file!(), line!());
src/libstd/macros.rs:47:        let (file, line) = _FILE_LINE;
src/libstd/macros.rs:69:            static _FILE_LINE: (&'static str, uint) = (file!(), line!());
src/libstd/macros.rs:70:            ::std::rt::begin_unwind_fmt(fmt, &_FILE_LINE)
