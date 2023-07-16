 rust
extern mod std;
use std::getopts::{optflag, getopts, opts_present, opt_present};

fn main() {
        let opts = ~[optflag("h"), optflag("t"), optflag("help"),
                     optflag("tandy")];

        match getopts(os::args(), opts) {
                Ok(ref matches) => {
                        // valid args
                        let mut test = ~[~"help", ~"h", ~"tandy", ~"t"];
                        for test.each | &x | {
                                io::println(fmt!("%8s: opt_present: %5?, opts_present: %5?",
                                                x, opt_present(matches, x), opts_present(matches, ~[x])));
                        }
                        io::print("\n");

                        // invalid args
                        test = ~[~"whatever", ~"w"];
                        for test.each | &x | {
                                io::println(fmt!("%8s: opts_present: %5?", x,
                                                opts_present(matches, ~[x])));
                        }
                        // (should fail)
                        for test.each | &x | {
                                io::println(fmt!("%8s: opt_present: %5?",
                                                x, opt_present(matches, x)));
                        }
                },
                Err(_) => ()
        };
}
