
% rustup default nightly-2017-11-01-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2017-11-01-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2017-11-01-x86_64-unknown-linux-gnu'

  nightly-2017-11-01-x86_64-unknown-linux-gnu unchanged - rustc 1.23.0-nightly (8b22e70b2 2017-10-31)

% rustc --version
rustc 1.23.0-nightly (8b22e70b2 2017-10-31)
% rustc issue-52126.rs
error[E0597]: `line` does not live long enough
  --> issue-52126.rs:43:5
   |
30 |         let v: Vec<&str> = line.split_whitespace().collect();
   |                            ---- borrow occurs here
...
43 |     }
   |     ^ `line` dropped here while still borrowed
44 | }
   | - borrowed value needs to live until here

error[E0597]: `line` does not live long enough
  --> issue-52126.rs:63:5
   |
50 |         let v: Vec<&str> = line.split_whitespace().collect();
   |                            ---- borrow occurs here
...
63 |     }
   |     ^ `line` dropped here while still borrowed
64 | }
   | - borrowed value needs to live until here

error: aborting due to 2 previous errors

% rustup default nightly-2017-11-02-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2017-11-02-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2017-11-02-x86_64-unknown-linux-gnu'

  nightly-2017-11-02-x86_64-unknown-linux-gnu unchanged - rustc 1.23.0-nightly (2be4cc040 2017-11-01)

% rustc --version
rustc 1.23.0-nightly (2be4cc040 2017-11-01)
% rustc issue-52126.rs
%
