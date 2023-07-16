rust
match $dst.fn write_fmt {
    write_fmt => {
        let result = write_fmt(format_args!($($arg)*));
        result
    }
}
