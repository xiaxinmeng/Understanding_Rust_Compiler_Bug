 rust
macro_rules! cond(
    ($(($pred:expr) $body:block)+ _ $default:block) => (
        $(if $pred $body )else+
        else $default
    );
    ($(($pred:expr) $body:block)+) => (
        $(if $pred $body )else+
    );
)

fn main() {
    let x = 1;
    cond!(
        (x < 0) { io::println(~"woops");   }
        (true ) { io::println(~"hullo");   }
        _       { io::println(~"bananas"); }
    )

    cond!(
        (x < 0) { io::println(~"woops");   }
        (true ) { io::println(~"hullo");   }
    )
}
