rust
#[derive(Debug)]
struct QuoteMatcher<'a>(&'a Vec<String>);

impl<'a> QuoteMatcher<'a> {
    fn intermediates(&'a self) -> impl Iterator<Item = Intermediate> + 'a {
        self.0.iter().enumerate().flat_map(move |(ix, string)| {
            let splitter = new_quote_splitter(&string).events();
            EachSplitter::Splitter { index: ix, splitter, seen_any: Some(false) }
        })
    }
}

impl<'a, I: Iterator<Item = (usize, char)>> QuoteSplitter<'a, I> {
    fn events(self) -> impl Iterator<Item = Event<'a>> {
        self.flat_map(|x| x)
    }
}

#[derive(Debug)]
enum EachSplitter<'a, I: Iterator<Item = Event<'a>> + 'a> {
    Splitter { splitter: I, seen_any: Option<bool>, index: usize },
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for EachSplitter<'a, I> {
    type Item = Intermediate;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

struct Event<'a>(&'a str);

struct Intermediate;

fn leaning_text(node: &str, rightmost: bool) -> Option<&str> {
    todo!()
}

#[derive(Debug)]
struct QuoteSplitter<'a, I: Iterator<Item = (usize, char)> + 'a> {
    string: &'a str,
    possibles: I,
}

fn new_quote_splitter<'a>(
    string: &'a str,
) -> QuoteSplitter<'a, impl Iterator<Item = (usize, char)> + 'a> {
    QuoteSplitter {
        string,
        possibles: string.char_indices().filter(|_| true),
    }
}

impl<'a, I: Iterator<Item = (usize, char)>> Iterator for QuoteSplitter<'a, I> {
    type Item = Thingo<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

struct Thingo<'a>(&'a ());

impl<'a> Iterator for Thingo<'a> {
    type Item = Event<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {}
