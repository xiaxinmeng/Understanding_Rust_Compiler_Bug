rust
    fn make_logger() -> ::slog::Logger {
        use slog::{Drain, Logger};
        use std::{io, sync::Mutex};
        Logger::root(
            Mutex::new(
                slog_term::CompactFormat::new(
                    slog_term::PlainSyncDecorator::new(io::stdout())
                ).build().fuse()
            ).fuse(),
            o!(),
        )
    }
