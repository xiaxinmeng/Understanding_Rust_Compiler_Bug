sh
$ git checkout master
$ git pull origin master     # ensure master is really up to date

$ git checkout allow-irrefutable_patterns 
$ git rebase master          # insert your changes on top of the latest master

$ # fix rebase conflict...
$ nano src/librustc_const_eval/check_match.rs
$ git add src/librustc_const_eval/check_match.rs 
$ git rebase --continue
$ # fix rebase conflict...
$ nano src/libsyntax/feature_gate.rs
$ git add src/libsyntax/feature_gate.rs  
$ git rebase --continue
