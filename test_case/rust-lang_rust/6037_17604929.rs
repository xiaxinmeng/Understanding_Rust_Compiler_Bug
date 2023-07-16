 rust
//
// A Scheme-style conditional that helps to improve code clarity in some instances when
// the `if`, `else if`, and `else` keywords obscure predicates undesirably.
//
// # Example
//
// ~~~
// let clamped =
//     if x > mx { mx }
//     else if x < mn { mn }
//     else { x };
// ~~~
//
// Using `cond!`, the above could be written as:
//
// ~~~
// let clamped = cond!(
//     | x > mx { mx }
//     | x < mn { mn }
//     _        { x  }
// );
// ~~~
//
// The optional default case is denoted by `_`.
//
macro_rules! cond (
    ($( | $pred:expr $body:block)+ _ $default:block ) => (
        $(if $pred $body else)+
        $default
    );
    // for if the default case was ommitted
    ( $( | $pred:expr $body:block )+ ) => (
        $(if $pred $body)else+
    );
)
