
% RUSTFLAGS=-Dwarnings cargo +nightly-2020-04-04 build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s

% RUSTFLAGS=-Dwarnings cargo +nightly-2020-04-04 doc
 Documenting repro v0.1.0 (/private/tmp/repro)
warning: unnecessary braces around block return value
  |
  = note: `#[warn(unused_braces)]` on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.82s
