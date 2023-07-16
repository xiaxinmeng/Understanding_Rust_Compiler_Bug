
error[E0308]: mismatched types
  --> operations\test-ops-client\src\main.rs:24:27
   |
8  | |   }
   | |                                                    ^
   | |                                                    |
   | |____________________________________________________expected enum `std::option::Option`, found struct `geom_kernel::Point3Msg`
   |                                                      help: try using a variant of the expected enum: `Some(Point3Msg{x: 0.0, y: 0.0, z: 0.0,})`
...
19 | |       let outbound = async_stream::stream! {
   | |  ____________________-
20 | | |         let input = CreateWallInput {
21 | | |             file: String::from("test_ops_client"),
22 | | |             user: String::from("test_ops_user"),
23 | | |             wall: Some(WallMsg {
24 |   |                 first_pt: Point3Msg {
   |  _|___________________________^
...    |
38 |   |         yield input;
39 |   |     };
   |   |_____- in this macro invocation
   |
   = note: expected enum `std::option::Option<geom_kernel::Point3Msg>`
            found struct `geom_kernel::Point3Msg`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
