rust
    /// Argument that must be passed *directly* to the linker
    ///
    /// These arguments need to be prepended with `-Wl`, when a GCC-style linker is used.
    fn linker_arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        if !self.is_ld {
            let mut os = OsString::from("-Wl,");
            os.push(arg.as_ref());
            self.cmd.arg(os);
        } else {
            self.cmd.arg(arg);
        }
        self
    }
