rust
        (fails == 0)
            .then_some(())
            .ok_or_else(|| anyhow!("Failed synchronizing {fails} exceptions."))
            .and_then(|_| self.commit_exceptions())
