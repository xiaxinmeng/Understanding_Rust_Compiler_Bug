 rust
<anon>:5:28: 5:29 error: borrowed value does not live long enough
<anon>:5         let s = ~"foo bar"; s.word_iter().collect::<~[&str]>()
                                     ^
<anon>:3:10: 8:1 note: borrowed pointer must be valid for the block at 3:10...
<anon>:3 fn main() {
<anon>:4     let r = {
<anon>:5         let s = ~"foo bar"; s.word_iter().collect::<~[&str]>()
<anon>:6     };
<anon>:7     println(fmt!("%?", r))
<anon>:8 }
<anon>:4:12: 6:5 note: ...but borrowed value is only valid for the block at 4:12
<anon>:4     let r = {
<anon>:5         let s = ~"foo bar"; s.word_iter().collect::<~[&str]>()
<anon>:6     };
error: aborting due to previous error
application terminated with error code 101
