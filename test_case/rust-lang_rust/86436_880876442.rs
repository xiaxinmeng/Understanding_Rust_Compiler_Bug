
> # Installs chosen set of extended tools if `extended = true`. By default builds
> # all extended tools except `rust-demangler`, unless the target is also being
> # built with `profiler = true`. If chosen tool failed to build the installation
> # fails. If `extended = false`, this option is ignored.
> #tools = ["cargo", "rls", "clippy", "rustfmt", "analysis", "src"] # + "rust-demangler" if `profiler`
> 