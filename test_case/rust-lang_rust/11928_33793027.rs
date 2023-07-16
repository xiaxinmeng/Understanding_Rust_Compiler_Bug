 rust
macro_rules! expr { ($e: expr) => { $e } }

macro_rules! spawn {
    ( $($body: tt)* ) => {
        expr!(::std::task::spawn(proc() { $($body)* }))
    }
}

fn main() {
     spawn! { foo(); bar() } // perfectly fine
}
