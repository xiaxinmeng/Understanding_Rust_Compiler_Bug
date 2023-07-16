
    pub fn output(&mut self) -> io::Result<Output> {
        self.inner.spawn(imp::Stdio::MakePipe, false).map(Child::from_inner)
            .and_then(|p| p.wait_with_output())
    }
