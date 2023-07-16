 rust
match foo {
    ref foo_addr => {
        static PRECOMPILED_FORMAT_STRING = ...;
        let arguments: &fmt::Arguments = unsafe { /* do things with foo_addr */ };
        fmt::writeln(logger, arguments)
    }
}
