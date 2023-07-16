rust
    fn get_max_line_num(&mut self, span: &MultiSpan, children: &[SubDiagnostic]) -> usize {
        let primary = self.get_multispan_max_line_num(span);
        children.iter()
            .map(|sub| self.get_multispan_max_line_num(&sub.span))
            .max()
            .unwrap_or(0)
            .max(primary)
    }
