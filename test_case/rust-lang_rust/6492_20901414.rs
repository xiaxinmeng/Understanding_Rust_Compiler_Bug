 rust
extern mod extra;
use extra::getopts::*;

fn main() {
    let args = std::os::args();

    let opts = ~[
        optopt("a"), optopt("arg")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)  => m,
        Err(f) => fail!(fail_str(f))
    };

    if opts_present(&matches, [~"a", ~"arg"]) {
        println(fmt!("Argument `%s`", opts_str(&matches, [~"a", ~"arg"])));
        return;
    }
}
