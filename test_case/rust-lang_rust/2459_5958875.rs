
// rustc --test long_strings.rs && ./long_strings
use std;

//#[test]
fn short()
{
    let text = "hello world";
    io::println(text);                        // hello world
    io::println(#fmt["%s", text]);        // hello world
    io::println(#fmt["%?", text]);        // "hello world"
    #error["%?", text];                    // rust: "\"hello world\""
}

#[test]
fn long()
{
    let mut text = "";
    iter::repeat(175u)
    {||
        text += " hello world";
    };
    io::println("1");
    io::println(text);    

    io::println("2");
    io::println(#fmt["%s", text]);

    io::println("3");
    io::println(#fmt["%?", text]);

    io::println("4");
    #error["%?", text];                    // all of the log macros are truncated
}
