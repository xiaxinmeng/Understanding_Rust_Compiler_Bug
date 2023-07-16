` rust
    pub fn read_regex(&mut self, regex: &Regex) -> LexResult<Token> {
        self.ensure_has_input()?;
        let (m_start, m_end) = match regex.find(self.remaining_source_code()) {
            Some(regex_match) if regex_match.start() == 0 =>
                (regex_match.start(),  regex_match.end()),
            _ => return Err(LexErr::ExpectedRegexMatch {
                position: self.position()
            }),
        };
        let start = self.position() + m_start;
        let end   = self.position() + m_end;
        self.position = end;
        Ok(Token::new(start..end))
    }
