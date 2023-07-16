rust
let mut __capture0 = None;
let mut __capture1 = None;
let mut __capture2 = None;
// `try_copy` copies it if it implements `Copy`
if !({ let __tmp = a; __capture0 = __tmp.try_copy(); __tmp }
   == { let __tmp = b; __capture1 = __tmp.try_copy(); __tmp }
   && { let __tmp = c; __capture2 = __tmp.try_copy(); __tmp }) {
    panic!("assertion failed: a == b && c\nwith expansion: {:?} == {:?} && {:?}", ...);
}
