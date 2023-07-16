
        .           impl Iterator for EscapeDefault {
        .               type Item = u8;
7,040,008 ( 2.39%)      fn next(&mut self) -> Option<u8> {
8,224,494 ( 2.80%)          self.range.next().map(|i| self.data[i as usize])
7,040,008 ( 2.39%)      }
