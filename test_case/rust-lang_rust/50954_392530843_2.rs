
May 25 22:28:02.573 INFO kablam! error[E0283]: type annotations required: cannot resolve `std::string::String: std::convert::AsRef<_>`
May 25 22:28:02.573 INFO kablam!   --> examples/demo.rs:60:50
May 25 22:28:02.573 INFO kablam!    |
May 25 22:28:02.573 INFO kablam! 60 |             let builds = travis.builds(repo.slug.as_ref());
May 25 22:28:02.573 INFO kablam!    |                                                  ^^^^^^
