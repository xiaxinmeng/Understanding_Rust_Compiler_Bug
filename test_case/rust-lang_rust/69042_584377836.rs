rust
        #[cfg(target_os = "fuchsia")]
        fuchsia::print_dso_context(self.fmt)?;
        Ok(())
