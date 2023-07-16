
D:\3rdparty\indicatif> cargo build --examples
   Compiling indicatif v0.10.3 (D:\3rdparty\indicatif)
warning: use of deprecated item 'rand::Rng::choose': use SliceRandom::choose ins
tead
  --> examples\yarnish.rs:82:27
   |
82 |             let pkg = rng.choose(PACKAGES).unwrap();
   |                           ^^^^^^
   |
   = note: #[warn(deprecated)] on by default

warning: use of deprecated item 'rand::Rng::choose': use SliceRandom::choose ins
tead
  --> examples\yarnish.rs:84:31
   |
84 |                 let cmd = rng.choose(COMMANDS).unwrap();
   |                               ^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 4.24s
