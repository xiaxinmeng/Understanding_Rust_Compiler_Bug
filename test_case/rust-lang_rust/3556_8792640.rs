
// rustc --test to_str.rs && ./to_str
extern mod std;
use io::{ReaderUtil, WriterUtil};

enum Token {
    Text(@~str),
    ETag(@~[~str], @~str),
    UTag(@~[~str], @~str),
    Section(@~[~str], bool, @~[Token], @~str, @~str, @~str, @~str, @~str),
    IncompleteSection(@~[~str], bool, @~str, bool),
    Partial(@~str, @~str, @~str),
}

#[cfg(test)]
fn check_strs(actual: &str, expected: &str) -> bool
{
    if actual != expected
    {
        io::stderr().write_line(fmt!("Found %s, but expected %s", actual, expected));
        return false;
    }
    return true;
}

#[test]
fn tester()
{
    assert check_strs(fmt!("%?", Text(@~"foo")), "Text(@~\"foo\")");
    assert check_strs(fmt!("%?", ETag(@~[~"foo"], @~"bar")), "ETag(@~[ ~\"foo\" ], @~\"bar\")");

    let t = Text(@~"foo");
    let u = Section(@~[~"alpha"], true, @~[t], @~"foo", @~"foo", @~"foo", @~"foo", @~"foo");
    let v = fmt!("%?", u);    // this is the line that causes the seg fault
    assert v.len() > 0;
}
