
rustc 1.16.0-nightly (5d994d8b7 2017-01-05)

error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in function call due to conflicting requirements
  --> src/entities/map.rs:84:61
   |
84 |             jobs.push(RenderJob::DrawMany(self.tx_star_fg, &self.starfield[..]));
   |                                                             ^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: Could not compile `megumin`.

To learn more, run the command again with --verbose.
