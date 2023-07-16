
% git clone https://gist.github.com/andreastt/498e3abe08a6a16123e99c02a77c2c73
Cloning into '498e3abe08a6a16123e99c02a77c2c73'...
remote: Counting objects: 4, done.        
remote: Compressing objects: 100% (3/3)           
remote: Compressing objects: 100% (3/3), done.        
remote: Total 4 (delta 0), reused 0 (delta 0), pack-reused 0        
Unpacking objects: 100% (4/4), done.
% cd 498e3abe08a6a16123e99c02a77c2c73/
% TERM=dumb cargo build
   Compiling foo v0.1.0 (file:///tmp/498e3abe08a6a16123e99c02a77c2c73)
[0m[1m[33mwarning[0m[0m[1m: unused variable: `bar`[0m
[0m [0m[0m[1m[38;5;12m--> [0m[0mfoo.rs:2:9[0m
[0m  [0m[0m[1m[38;5;12m|[0m
[0m[1m[38;5;12m2[0m[0m [0m[0m[1m[38;5;12m| [0m[0m    let bar = "42";[0m
[0m  [0m[0m[1m[38;5;12m| [0m[0m        [0m[0m[1m[33m^^^[0m[0m [0m[0m[1m[33mhelp: consider using `_bar` instead[0m
[0m  [0m[0m[1m[38;5;12m|[0m
[0m  [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: #[warn(unused_variables)] on by default[0m

    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
