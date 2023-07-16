 rust
static PRECOMPILED_FORMAT_STRING = ...;
match foo {
    ref foo_addr => {
        let arguments: &fmt::Arguments = unsafe { /* do things with foo_addr */ };
        fmt::writeln(logger, arguments)
    }
}
