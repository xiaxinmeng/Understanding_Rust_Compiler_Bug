
~/Documents
❯ git clone https://github.com/jrobsonchase/proc_macro_problem
Cloning into 'proc_macro_problem'...
remote: Enumerating objects: 13, done.
remote: Total 13 (delta 0), reused 0 (delta 0), pack-reused 13
Unpacking objects: 100% (13/13), 1.29 KiB | 55.00 KiB/s, done.
~/Documents
❯ cd proc_macro_problem\baz
proc_macro_problem/baz on 📙 master is 📦 v0.1.0 via 🦀 v1.46.0
❯ cargo test
    Updating crates.io index
   Compiling proc-macro2 v0.4.30
   Compiling unicode-xid v0.1.0
   Compiling syn v0.15.44
   Compiling quote v0.6.13
   Compiling foo v0.1.0 (C:\Users\steve\Documents\proc_macro_problem\foo)
   Compiling baz v0.1.0 (C:\Users\steve\Documents\proc_macro_problem\baz)
    Finished test [unoptimized + debuginfo] target(s) in 11.61s
     Running target\debug\deps\baz-01d343961c658c59.exe

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests baz

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
