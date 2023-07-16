 rust
#[cfg(not(works))]
pub trait LineFormatter<'a> {
    type Iter: Iterator<Item=&'a str> + 'a;
    fn iter(&'a self, line: &'a str) -> Self::Iter;

    fn dimensions(&'a self, line: &'a str) {
        for _grapheme in self.iter(line) {
        }
    }
}

#[cfg(works)]
pub trait LineFormatter {
    type Iter: Iterator<Item=&'static str>;
    fn iter<'a>(&'a self, line: &'a str) -> Self::Iter;

    fn dimensions<'a>(&'a self, line: &'a str) {
        for _grapheme in self.iter(line) {
        }
    }
}

fn main() {}
