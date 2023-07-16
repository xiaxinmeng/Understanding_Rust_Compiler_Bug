rust
#[derive(Debug, Clone, Copy)]
pub struct Location<'a> {
    pub filename: &'a str,
    pub start: LocationHalf,
    pub end: LocationHalf,
}

#[derive(Debug, Clone, Copy)]
pub struct LocationHalf {
    pub line: u32,
    pub column: u32,
}

impl Location<'_> {
    /// Returns an iterator over the line numbers of this location.
    pub fn line_numbers(self) -> impl Iterator<Item = u32> {
        self.start.line..=self.end.line
    }

    /// Returns an iterator over the line numbers and lines of this location.
    pub fn lines<'a>(self, source: &'a str) -> impl Iterator<Item = (u32, &'a str)> {
        let lines = source.split('\n');
        lines.enumerate().filter_map(|(i, line): (usize, &'a str)| {
            if self.start.line as usize <= i && i <= self.end.line as usize {
                Some((i as u32 + 1, line))
            } else {
                None
            }
        })
    }
}

