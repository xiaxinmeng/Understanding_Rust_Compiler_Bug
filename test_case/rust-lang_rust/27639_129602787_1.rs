
<anon>:3:9: 6:10 error: non-exhaustive patterns: `Some(_)` not covered [E0004]
<anon>:3         match $ex {
<anon>:4             Some(ouch) => println!("{}", ouch),
<anon>:5             None => println!("none")
<anon>:6         }
