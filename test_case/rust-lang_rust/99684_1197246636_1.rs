rust
            write!(
                writer,
                "{}{} ",
                writer.bold().paint(meta.target()),
                writer.dimmed().paint(":")
            )?;
