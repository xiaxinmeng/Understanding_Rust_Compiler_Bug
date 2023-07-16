rust
#![allow(incomplete_features)]
#![feature(exclusive_range_pattern, half_open_range_patterns, inline_const)]

fn main() {
    for x in -9 + 1..=(9 - 2) {
        // if let n @ 2..3|4 = x {
        // ~^ error[E0408]: variable `n` is not bound in all patterns
        //     println!("n doesn't bind, so we don't get: {}", n);
        // } else
        if let 2..3 | 4 = x {
            println!("if lettable: {}", x);
        }
        match x {
            // 0..5+1 => println!("got: {}", x),
            //~^ error: expected one of `=>`, `if`, or `|`, found `+`
            // 0..=(5+1) => println!("got: {}", x),
            //~^ error[E0586]: inclusive range with no end
            //~| error: expected one of `=>`, `if`, or `|`, found `(`
            // 0-2..=2 => println!("match on {}"),
            //~^ error: expected one of `...`, `..=`, `..`, `=>`, `if`, or `|`, found `+`
            1 | -3..0 => println!("the first or-pattern got: {}", x),
            x @ (0..5 | 6) => println!("{} arrived in the second or-pattern", x),
            x @ 0..const { 5 + 1 } => println!("inline const only matches {}", x),
            x @ -5.. => println!("{}.. range from", x),
            x @ ..-7 => println!("..range to {}", x),
            x => println!("{} hit bottom", x),
        }
    }
}
