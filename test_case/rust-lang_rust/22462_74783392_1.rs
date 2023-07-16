
<anon>:3:9: 5:10 error: non-exhaustive patterns: `_` not covered [E0004]
<anon>:3         match $e {
<anon>:4             x => println!("{}", x),
<anon>:5         }
<anon>:1:1: 7:2 note: in expansion of foo!
<anon>:11:5: 11:13 note: expansion site
<anon>:3:9: 5:10 help: pass `--explain E0004` to see a detailed explanation
