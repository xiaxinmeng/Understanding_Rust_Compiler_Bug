rust
enum Inner {
    Alpha(Backtrace),
    Beta,
}

impl Inner {
    #[provides]
    const PROVIDES = [Self::whatever_goes_here_for_enum_variants];
}

struct Outer { inner: Inner, status: ExitCode };

impl Outer {
    #[provides]
    const PROVIDES = [
        ...Inner::PROVIDES, // Copy everything from `Inner`... somehow
        Backtrace, // Or maybe we have to duplicate all the types from `Inner`?
        Self::status, // Add our own data
        // How can we choose to supersede `Inner`'s data with our own (or vice-versa)?
    ];
}
