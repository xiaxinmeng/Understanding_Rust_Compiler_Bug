
lunch-box. rustc ~/tmp/weird.rs -Znll -Zborrowck=mir -Znll-dump-cause
error[E0597]: borrowed value does not live long enough
 --> /home/nmatsakis/tmp/weird.rs:4:16
  |
4 |     let line = std::io::stdin().lock().lines().next().unwrap().unwrap();
  |                ^^^^^^^^^^^^^^^^                                         - temporary value only lives until here
  |                |
  |                temporary value does not live long enough
  |
  = note: borrowed value must be valid for lifetime '_#2r...
