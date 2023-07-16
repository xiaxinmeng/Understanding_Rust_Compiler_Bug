
rustc 1.16.0-nightly (5d994d8b7 2017-01-05)

error: `world` does not live long enough
   --> src/main.rs:148:1
    |
133 |         world.draw(&mut render_jobs);
    |         ----- borrow occurs here
...
148 | }
    | ^ `world` dropped here while still borrowed
    |
    = note: values in a scope are dropped in the opposite order they are created

error: aborting due to previous error

error: Could not compile `megumin`.

To learn more, run the command again with --verbose.
