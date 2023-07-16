
// Run with `export RUST_LOG=hello && ./bin/hello`
import io;

fn is_print(ch: char) -> bool
{
    ret ch >= '_' && ch <= '~';
}

fn munge_chars(chars: [char]) -> str
{
    let bullet = '\u2022';

    let mut value = "";
    str::reserve(value, vec::len(chars));
    vec::iter(chars) {|ch| str::push_char(value, if is_print(ch) {ch} else {bullet});}
    ret value;
}

fn main() 
{
    let s = munge_chars(['h', 'e', 'l', 'l', 'o', '\u0003']);

    // With the terminal on Mac OSX this prints the following two lines:
    // hello<bullet>
    // rust: "hello\xffffffe2\xffffff80\xffffffa2"
    io::println(s);
    #info("%s", s);
}
