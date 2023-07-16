rust
    pub fn non_exhaustive(&mut self) -> &mut DebugStruct<'a, 'b> {
        self.result = self.result.and_then(|_| {
            if self.is_pretty() {
                if !self.has_fields {
                    self.fmt.write_str(" {\n")?;
                }
                let mut slot = None;
                let mut state = Default::default();
                let mut writer = PadAdapter::wrap(&mut self.fmt, &mut slot, &mut state);
                writer.write_str("..\n")
            } else {
                let prefix = if self.has_fields { ", " } else { " { " };
                self.fmt.write_str(prefix)?;
                self.fmt.write_str("..")
            }
        });
        self.has_fields = true;
        self
    }

    pub fn finish_non_exhaustive(&mut self) -> fmt::Result {
        self.non_exhaustive().finish()
    }
