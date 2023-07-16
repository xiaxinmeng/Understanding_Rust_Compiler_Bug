
error[E0308]: mismatched types
   --> src\librustc_trans\back\link.rs:131:10
    |
131 |         (tool.to_command(), envs)
    |          ^^^^^^^^^^^^^^^^^ expected struct `back::command::Command`, found struct `std::process::Command`
    |
    = note: expected type `back::command::Command`
               found type `std::process::Command`
error: aborting due to previous error
error: Could not compile `rustc_trans`.

