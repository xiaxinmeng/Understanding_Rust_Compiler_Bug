rust
        } else if self.last_unexpected_token_span == Some(self.token.span) {
            FatalError.raise();
        }
