
        (|buf : &mut fmt::Formatter, args : std::fmt::Arguments<'_>| {
            let i = args.to_string().len();
            buf.write_fmt(args)?;

            for _ in 0..(128 - i) {
                write!(buf, " ")?;
            }
            writeln!(buf, "{}:{}", file, line)
        })(buf, format_args!("[{}][{}] - {}", level, target, args))
